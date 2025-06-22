#ifndef H_CRYPTOPALS_UTILS
#define H_CRYPTOPALS_UTILS

#include <stdint.h>

#define u8  uint8_t
#define u16 uint16_t
#define u32 uint32_t

#define s8  int8_t
#define s16 int16_t
#define s32 int32_t

u32 hexStringToBase64(u8* pInBuf, u32 len, u8* pOutBuf);
void printBase64(u8* inBuf, u32 len);
u32 convertStringToHex(char* str);

#endif //H_CRYPTOPALS_UTILS