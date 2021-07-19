# vtf-tools
A set of small tools built in rust for management and checking VTF sprays


### convert-spray

Converts a given file (*.dat directly from server downloads folder) or *.vtf into *.png

`$ ./convert-spray file.dat dest.png`

### vtf-base64

Converts a given file (same as above) to PNG and prints out its base64

`$ ./vtf-base64 file.vtf`

### test-spray

Does all the steps above, then sends the base64 to google vision safe browsing api to check for sexual content.

Requires environment variable `SAFESEARCH_API_KEY`

Will return the string value of the "adult" result which generally has values of `UNLIKELY`, `LIKELY`, `VERY_LIKELY` or more

`$ ./test-spray file.vtf`


