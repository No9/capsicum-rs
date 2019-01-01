#include <stdio.h>
#include <libcasper.h>
#include <sys/capsicum.h>

#include "cap_dns_wrap.h"

cap_channel_t *capcas, *capdns ;
    
int init_cap_dns()
{
    enum Status {SUCCESS, EINITFAIL, ESVCOPENFAIL}; 
	
    /*	Open capability	to the Casper. */
    capcas = cap_init();
    if	(capcas	== NULL) {
        return EINITFAIL;
    }
	 
	capdns = cap_service_open(capcas, "system.dns");
    if	(capdns	== NULL) {
        return ESVCOPENFAIL;
    }
    /*	Close Casper capability, we don't need it anymore. */
    cap_close(capcas);
    return SUCCESS;
}