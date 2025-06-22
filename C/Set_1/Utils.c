#include "Utils.h"
#include <string.h>

static const u8 tableBase64[64] = {
    'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','0','1','2','3','4','5','6','7','8','9','+','/'
};

#define BASE64_MASK_1   0xFC
#define BASE64_MASK_2_1 0x03
#define BASE64_MASK_2_2 0xF0
#define BASE64_MASK_3_1 0x0F
#define BASE64_MASK_3_2 0xC0
#define BASE64_MASK_4   0x3F

u8 base64Sm(u8* pIn, u8 state)
{
    u8 val = *pIn;
    switch(state)
    {
        case 0 :
            return (*pIn & BASE64_MASK_1) >> 2;
            break;
        
        case 1 :
            return ((*pIn & BASE64_MASK_2_1) << 4)  | ((*(pIn+1) & BASE64_MASK_2_2) >> 4);
            break;
        
        case 2 :
            return ((*pIn & BASE64_MASK_3_1) << 2)  | ((*(pIn+1) & BASE64_MASK_3_2) >> 6);
            break;

        case 3 :
            return *pIn & BASE64_MASK_4;
            break;

        default:
            return 0;
    }
}

u32 hexStringToBase64(u8* pInBuf, u32 len, u8* pOutBuf)
{
    // u32 i = 0;
    // u32 outIdx = 0;
    // for(i = 0; i< len; i+=3)
    // {
    //     pOutBuf[outIdx] = (pInBuf[i] & BASE64_MASK_1) >> 2;
    //     pOutBuf[outIdx+1] = ((pInBuf[i] & BASE64_MASK_2_1) << 4)  | ((pInBuf[i+1] & BASE64_MASK_2_2) >> 4);
    //     pOutBuf[outIdx+2] = ((pInBuf[i+1] & BASE64_MASK_3_1) << 2)  | ((pInBuf[i+2] & BASE64_MASK_3_2) >> 6);
    //     pOutBuf[outIdx+3] = pInBuf[i+2] & BASE64_MASK_4 ;

    //     outIdx += 4;
    // }

    u32 i = 0;
    u32 outIdx = 0;
    while(i<len)
    {
        pOutBuf[outIdx]   = base64Sm(&pInBuf[i], outIdx&3);
        //pOutBuf[outIdx+1] = base64Sm(&pInBuf[i], 1);
        //pOutBuf[outIdx+2] = base64Sm(&pInBuf[i+1], 2);
        //pOutBuf[outIdx+3] = base64Sm(&pInBuf[i+2], 3);
        
        i = (outIdx&0x03) ? i+1 : i;
        outIdx += 1;
    }

    return outIdx;
}

u8 hex_ctoi(char c)
{
    if (c >= 'a' && c <= 'f')
    {
        return 10 + c - 'a';
    }
    if (c >= '0' && c <= '9')
    {
        return c - '0';
    }

    return -1;
}

static const u8 charToHexTable[0x37] = 
{   0,1,2,3,4,5,6,7,8,9, 
    [0x11] = 0xA, [0x12] = 0xB , [0x13] = 0xC , [0x14] = 0xD, [0x15] = 0xE, [0x16] = 0xF, 
    [0x31] = 0xa, [0x32] = 0xb , [0x33] = 0xc , [0x34] = 0xd, [0x35] = 0xe, [0x36] = 0xf, 
};

u32 convertStringToHex(char* str)
{
    u32 len = strlen(str);
    u32 newLen = len >> 1;
    for(u32 i = 0; i< len; i++)
    {
        str[i] -= 0x30;
        str[i] = charToHexTable[str[i]];
    }

    u32 readIdx = 0;
    for(u32 i = 0; i< newLen; i++)
    {
        str[i] = (str[readIdx] << 4) | str[readIdx+1];
        readIdx +=2;
    }

    if(len & 0x1)
    {
        str[newLen] = str[len-1];
        newLen += 1;
    }

    return newLen;
}

void printBase64(u8* inBuf, u32 len)
{
    for(int i=0;i<len;i++)
    {
        printf("%c",tableBase64[inBuf[i]]);
    }
}


