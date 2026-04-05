
// retro basic

// lister.c   23 Oct 2018
//    generate listing from B-code, (disassembling)
//    P Chongstitvatana

#include "lister.h"

#define head(x) bcode[(x)]
#define tail(x) bcode[(x)+1]


int bcode[MAXCODE];					// b-code stored as array of int

// read input file into bcode[]
void readinfile(char *fname) {
    FILE *fp;
    int n, i;

    fp = fopen(fname,"rt");
    if( fp == NULL ) {
        printf("cannot open : %s\n",fname);
        exit(-1);
    }
	i = 0;
	while( fscanf(fp,"%d",&n) != (int)EOF ){
		bcode[i] = n;
		i++;
	}
    fclose(fp);
}

void dumpbcode(void){
	int i;
	i = 0;
	while( head(i) != 0 ){
		printf("%d %d ",head(i),tail(i));
		i += 2;
	}
	printf("\n");
}

// print id, check that it is in range A..Z 1..26
// ascii of A is 65
void print_id(int value){
	if( value >= 1 && value <= 26 )
		printf("%c ",value+64);
	else
		printf("undef ");
}

// print if bcode is valid
void listing(void){
	int i, type, value;
	i = 0;
	while( head(i) != 0 ){
		type = head(i);
		value = tail(i);
		switch(type){
		case t_line: printf("\n%d ",value); break;
		case t_id: print_id(value); break;
		case t_const: printf("%d ",value); break;
		case t_if: printf("IF "); break;
		case t_goto: printf("GOTO %d ",value); break;
		case t_print: printf("PRINT "); break;
		case t_stop: printf("STOP ");
		case t_op:
			switch(value){
			case op_plus: printf("+ "); break;
			case op_minus: printf("- "); break;
			case op_lt: printf("< "); break;
			case op_eq: printf("= "); break;
			}
			break;
		}
		i += 2;
	}
}

char source[80];	// input file name

int main( int argc, char *argv[] ){

	if( argc < 2 ) {
		printf("usage : lister source\n");
		exit(-1);
	}
	strcpy(source,argv[1]);
	readinfile(source);
	dumpbcode();
	listing();
	return 0;
}

