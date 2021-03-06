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

Requires environment variable `SAFESEARCH_API_KEY` or pass apikey as second parameter

Will return the string value of the "adult", "medical" and "racy" result which generally has values of `VERY_UNLIKELY`, `UNLIKELY`, `POSSIBLE`, `LIKELY`, `VERY_LIKELY` and `UNKNOWN` and the numeric form starting from VERY_UNLIKELY (0) to VERY_LIKELY (4) and unknown being -1.

`$ ./test-spray file.vtf [apikey]`

```
adult=LIKELY=3
racy=VERY_LIKELY=4
medical=UNLIKELY=0
```
