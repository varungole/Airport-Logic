#include<stdio.h>


bool boarding()
{
    return true;
}
int main()
{
 printf("Welcome to Mumbai Airport! \n");
 printf("Tell us the Flight you are catching today \n");

 printf("1. Cathy Pacific 2. Etihad 3.Air France \n");
 int option;
 scanf("%d" , &option);
 
 if(option == 1)
 {
    printf("Thanks for Choosing Cathy Pacific");

    printf("Please proceed to Boarding");



    
 }
 return 0;    
}