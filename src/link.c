#include "fnmatch.h"

int fnm_nomatch = FNM_NOMATCH; /* Match failed. */

int fnm_noescape = FNM_NOESCAPE; /* Disable backslash escaping. */
int fnm_pathname = FNM_PATHNAME; /* Slash must be matched by slash. */
int fnm_period = FNM_PERIOD;     /* Period must be matched by period. */

/* A GNU extension: the pattern is matched ignoring a case */
#ifdef FNM_CASEFOLD
int fnm_casefold = FNM_CASEFOLD;
#else
int fnm_casefold = 0;
#endif
