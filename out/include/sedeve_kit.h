#pragma once

#include "_sedeve_kit.h"

#ifdef ENABLE_DTM

#define INPUT_ACTION(name, source, dest, action) { automata_action(name, C_ACTION_END, C_ACTION_INPUT, source, dest, action); }

#define OUTPUT_ACTION(name, source, dest, action) { automata_action(name, C_ACTION_BEGIN, C_ACTION_OUTPUT, source, dest, action); }

#define INTERNAL_ACTION_BEGIN(name, source, dest, action) { automata_action(name, C_ACTION_BEGIN, C_ACTION_INTERNAL, source, dest, action); }

#define INTERNAL_ACTION_END(name, source, dest, action) { automata_action(name, C_ACTION_END, C_ACTION_INTERNAL, source, dest, action); }


#else

#define INPUT_ACTION(_name, _source, _dest, _action)  ;

#define OUTPUT_ACTION(_name, _source, _dest, _action) ;

#define INTERNAL_ACTION_BEGIN(_name, _source, _dest, _action) ;

#define INTERNAL_ACTION_END(_name, _source, _dest, _action) ;


#endif