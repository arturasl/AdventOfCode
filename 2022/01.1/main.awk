/[[:digit:]]+/{c+=$0;r=r<c?c:r;next}
{c=0}
END{print r}
