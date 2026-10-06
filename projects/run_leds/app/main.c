/*==============================================================================
 * Пример работы GPIO для K1921VG1T
 *------------------------------------------------------------------------------
 * НИИЭТ, Александр Дыхно <dykhno@niiet.ru>
 *==============================================================================
 * ДАННОЕ ПРОГРАММНОЕ ОБЕСПЕЧЕНИЕ ПРЕДОСТАВЛЯЕТСЯ «КАК ЕСТЬ», БЕЗ КАКИХ-ЛИБО
 * ГАРАНТИЙ, ЯВНО ВЫРАЖЕННЫХ ИЛИ ПОДРАЗУМЕВАЕМЫХ, ВКЛЮЧАЯ ГАРАНТИИ ТОВАРНОЙ
 * ПРИГОДНОСТИ, СООТВЕТСТВИЯ ПО ЕГО КОНКРЕТНОМУ НАЗНАЧЕНИЮ И ОТСУТСТВИЯ
 * НАРУШЕНИЙ, НО НЕ ОГРАНИЧИВАЯСЬ ИМИ. ДАННОЕ ПРОГРАММНОЕ ОБЕСПЕЧЕНИЕ
 * ПРЕДНАЗНАЧЕНО ДЛЯ ОЗНАКОМИТЕЛЬНЫХ ЦЕЛЕЙ И НАПРАВЛЕНО ТОЛЬКО НА
 * ПРЕДОСТАВЛЕНИЕ ДОПОЛНИТЕЛЬНОЙ ИНФОРМАЦИИ О ПРОДУКТЕ, С ЦЕЛЬЮ СОХРАНИТЬ ВРЕМЯ
 * ПОТРЕБИТЕЛЮ. НИ В КАКОМ СЛУЧАЕ АВТОРЫ ИЛИ ПРАВООБЛАДАТЕЛИ НЕ НЕСУТ
 * ОТВЕТСТВЕННОСТИ ПО КАКИМ-ЛИБО ИСКАМ, ЗА ПРЯМОЙ ИЛИ КОСВЕННЫЙ УЩЕРБ, ИЛИ
 * ПО ИНЫМ ТРЕБОВАНИЯМ, ВОЗНИКШИМ ИЗ-ЗА ИСПОЛЬЗОВАНИЯ ПРОГРАММНОГО ОБЕСПЕЧЕНИЯ
 * ИЛИ ИНЫХ ДЕЙСТВИЙ С ПРОГРАММНЫМ ОБЕСПЕЧЕНИЕМ.
 *
 *                              2025 АО "НИИЭТ"
 *==============================================================================
 */
/*------------------------------------------------------------------------------
 * Local copy of the NIIET example, modified for this repository:
 *   SYSCLK is printed in the main() loop with a longer delay, so the UART output
 *   is continuous (the original prints it once at start-up and is easy to miss).
 * The original header above is kept unchanged.
 *------------------------------------------------------------------------------
 */

//-- Includes ------------------------------------------------------------------
#include <K1921VG1T.h>
#include <system_k1921vg1t.h>
#include "bsp.h"
#include "retarget.h"

//-- Defines -------------------------------------------------------------------

void TMR0_IRQHandler();

void TMR0_init(uint32_t period)
{
  RCU->CGCFGAPB1_bit.TMR0EN = 1;
  RCU->RSTDISAPB1_bit.TMR0EN = 1;

  //Записываем значение периода
  TMR0->PERIOD = period-1;
  //Выбираем режим счета от 0 до значения PERIOD
  TMR0->CTRL_bit.MODE = 1;
  //Разрешаем прерывание по совпадению значения счетчика и CAPCOM[0]
  TMR0->IM = 1;

  // Настраиваем обработчик прерывания для TMR32
  PLIC_SetIrqHandler (Plic_Mach_Target, IsrVect_IRQ_TMR0, TMR0_IRQHandler);
  PLIC_SetPriority   (IsrVect_IRQ_TMR0, 0x1);
  PLIC_SetMode		 (IsrVect_IRQ_TMR0,PLIC_IRQMODE_HILEVEL);
  PLIC_IntEnable     (Plic_Mach_Target, IsrVect_IRQ_TMR0);

  SIU->TMREN_bit.TMR0EN = 1;
}

//-- Peripheral init functions -------------------------------------------------
void periph_init()
{
	SystemInit();
	SystemCoreClockUpdate();
	BSP_LED_Init();
	retarget_init();
	printf("K1921VG1T SYSCLK = %d MHz\n",(unsigned int)(SystemCoreClock / 1000000));
	printf("  Start RunLeds\n");
}

//--- USER FUNCTIONS ----------------------------------------------------------------------

void delay()
{
	uint32_t i;
	for(i=0;i<20000000;i++) __asm("nop");   // original value: 100000
}

volatile uint32_t led_shift;
//-- Main ----------------------------------------------------------------------
int main(void)
{
  periph_init();
  TMR0_init(SystemCoreClock>>4);
  InterruptEnable();
  led_shift = LED0_MSK;
  uint32_t n = 0;
  while(1)
  {
    // print in the loop: continuous output
    printf("SYSCLK = %d MHz, tick %d\n",
           (unsigned int)(SystemCoreClock / 1000000),
           (unsigned int)(n++));
    delay();
  }

  return 0;
}


//-- IRQ INTERRUPT HANDLERS ---------------------------------------------------------------
void TMR0_IRQHandler()
{
  BSP_LED_Toggle(led_shift);
  led_shift = led_shift << 1;
  if(led_shift > LED3_MSK) led_shift = LED0_MSK;
  //Сбрасываем флаг прерывания таймера
  TMR0->IC = 1;
}
