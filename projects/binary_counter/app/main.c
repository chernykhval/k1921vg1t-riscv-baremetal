#include "K1921VG1T.h"

int main(void) {
    RCU->CGCFGAHB_bit.GPIOCEN = 1;
	RCU->RSTDISAHB_bit.GPIOCEN = 1;
    RCU->CGCFGAHB_bit.GPIOBEN = 1;
	RCU->RSTDISAHB_bit.GPIOBEN = 1;

    GPIOC->OUTENSET = 0x0000F000;

    uint32_t prev = 8;
    uint32_t counter = 0;

    while(1)
    {   
        uint32_t now = (GPIOB->DATA & 0x8);

        if (now == 0 && prev == 8) {
            counter = (counter + 1) % 16;
            uint32_t mask = counter << 12;
            GPIOC->DATAOUT = mask;
        }

        prev = now;
    }

    return 0;
}
