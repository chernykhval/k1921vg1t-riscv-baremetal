#include "K1921VG1T.h"

#define MAX_DUTY 100
#define MASK 0x00001000

int main(void) {
    RCU->CGCFGAHB_bit.GPIOCEN = 1;
	RCU->RSTDISAHB_bit.GPIOCEN = 1;

    GPIOC->OUTENSET = 0x0000F000;

    int32_t duty;

    while(1)
    {   
        GPIOC->DATAOUTSET = 0x00002000;

        for (duty = 0; duty <= MAX_DUTY; duty++) {
            for (int rep = 0; rep < 50; rep++) {
                int b = (duty * duty) / MAX_DUTY;
                GPIOC->DATAOUTSET = MASK;
                for (volatile int i=0; i<b; i++);
                GPIOC->DATAOUTCLR = MASK;
                for (volatile int i=0; i<(MAX_DUTY-b); i++);
            }
        }

        GPIOC->DATAOUTCLR = 0x00002000;

        for (duty = MAX_DUTY; duty >= 0; duty--) {
            for (int rep = 0; rep < 50; rep++) {
                int b = (duty * duty) / MAX_DUTY;
                GPIOC->DATAOUTSET = MASK;
                for (volatile int i=0; i<b; i++);
                GPIOC->DATAOUTCLR = MASK;
                for (volatile int i=0; i<(MAX_DUTY-b); i++);
            }
        }
    }

    return 0;
}
