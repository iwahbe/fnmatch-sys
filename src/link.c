#include "fnmatch.h"

int fnm_nomatch = FNM_NOMATCH; /* Match failed. */

int fnm_noescape = FNM_NOESCAPE; /* Disable backslash escaping. */
int fnm_pathname = FNM_PATHNAME; /* Slash must be matched by slash. */
int fnm_period = FNM_PERIOD;     /* Period must be matched by period. */
