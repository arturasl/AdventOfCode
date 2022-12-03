/[[:digit:]]+/{c+=$0;a[l]=c>a[l]?c:a[l];next}
{l++;c=0}
END{asort(a);print a[l+1]+a[l]+a[l-1]}
