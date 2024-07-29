````C

#include <stdio.h>

int main() {

    int num1;
    int num2;
    int op;

while (op !=9){

    printf("Hello! please pick your operation (1-9):\n1: Addition\n2: Subtraction\n3: Multiplication\n4: Division\n5: Remainder\n6: Power \n9: To Quit!\n");
    scanf("%d", &op);
    if (op == 1){
        printf("Please pick your first number: ");
        scanf("%d",&num1);
        printf("Please pick your second number: ");
        scanf("%d",&num2);
        int result = num1 + num2;
        printf("Addition results: %d + %d = %d \n\n",num1,num2,result);

    }  else if (op == 2){
        printf("Please pick your first number: ");
        scanf("%d",&num1);
        printf("Please pick your second number: ");
        scanf("%d",&num2);
        int result = num1 - num2;
        printf("Subtraction results: %d - %d = %d \n\n",num1,num2,result);

    } else if ( op == 3){
        printf("Please pick your first number: ");
        scanf("%d",&num1);
        printf("Please pick your second number: ");
        scanf("%d",&num2);
        int result = num1 * num2;
        printf("Multiplication results: %d * %d = %d \n\n",num1,num2,result);

    } else if ( op == 4){
        printf("Please pick your first number: ");
        scanf("%d",&num1);
        printf("Please pick your second number: ");
        scanf("%d",&num2);
        int result = num1 / num2;
        printf("Division results: %d / %d = %d \n\n",num1,num2,result);

    } else if ( op == 5){
        printf("Please pick your first number: ");
        scanf("%d",&num1);
        printf("Please pick your second number: ");
        scanf("%d",&num2);
        int result = num1 % num2;
        printf("Remainder results: %d %% %d = %d \n\n",num1,num2,result);

    } else if (op ==6){
        int result = 1;
        int tempPow;
        printf("Enter a number: ");
        scanf("%d",&num1);
        printf("Enter a POWER: ");
        scanf("%d", &num2);
        tempPow = num2;
        while(num2 >0){
            result = result *num1;
            num2--;

        }
        printf("%d in the power of %d = %d \n\n", num1,tempPow,result);


    } else if (op ==9){
        printf("Thank you! Goodbye\n\n");
        break;

    } else {
        printf("Pick a number 1 - 5!\n\n");

        }
    }
    return 0;
}
```c
````
