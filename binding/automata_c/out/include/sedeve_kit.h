#pragma once

#include "_sedeve_kit.h"

constexpr static const uint64_t C_ACTION_BEGIN = 0;

constexpr static const uint64_t C_ACTION_END = 1;

constexpr static const uint64_t C_ACTION_SETUP = 0;

constexpr static const uint64_t C_ACTION_CHECK = 1;

constexpr static const uint64_t C_ACTION_INPUT = 2;

constexpr static const uint64_t C_ACTION_OUTPUT = 3;

constexpr static const uint64_t C_ACTION_INTERNAL = 4;


#ifdef ENABLE_DTM

#define INPUT_ACTION(name, source, dest, action) { automata_action(name, C_ACTION_INPUT, C_ACTION_END, source, dest, action); }

#define OUTPUT_ACTION(name, source, dest, action) { automata_action(name, C_ACTION_OUTPUT, C_ACTION_BEGIN, source, dest, action); }

#define INTERNAL_ACTION_BEGIN(name, source, dest, action) { automata_action(name, C_ACTION_INTERNAL, C_ACTION_BEGIN, source, dest, action); }

#define INTERNAL_ACTION_END(name, source, dest, action) { automata_action(name, C_ACTION_INTERNAL, C_ACTION_END, source, dest, action); }


#else

#define INPUT_ACTION(_name, _source, _dest, _action)  ;

#define OUTPUT_ACTION(_name, _source, _dest, _action) ;

#define INTERNAL_ACTION_BEGIN(_name, _source, _dest, _action) ;

#define INTERNAL_ACTION_END(_name, _source, _dest, _action) ;


#endif