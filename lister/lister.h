// retro basic

// lister.h     23 Oct 2018

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define   PRIVATE   static


#define MAXCODE	10000	// max bcode size

// B-code type
#define t_line   10
#define t_id     11
#define t_const  12
#define t_if     13
#define t_goto   14
#define t_print  15
#define t_stop   16
#define t_op     17

#define op_plus		1
#define op_minus	2
#define op_lt		3
#define op_eq		4


