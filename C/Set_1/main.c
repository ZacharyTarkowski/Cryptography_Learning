#include <stdio.h>
#include <string.h>

#include "Utils.h"


int main()
{
    char string[100] = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    u32 newLen = convertStringToHex(string);
    u8 result[100];
    u32 resultLen = hexStringToBase64(string, newLen, result);
    printBase64(result,resultLen);
    printf("hello world");
    return 0;
}