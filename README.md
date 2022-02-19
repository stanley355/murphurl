# Shorten URL
Powerful and Fastest Short URL creator built in Rust

## Docs
1. To create shortened URL:\
Endpoint (POST): `https://morphurl.herokuapp.com/api/v1`\
Request Body (JSON):\
- origin_url (String): https://... (Required),\
- hashed_url (String): (Empty string - Required) 
- custom_url (String): https://... (Required, if you don't want to have any custom URL just put "" ),\

Return body (JSON):\
-  origin_url (String): https://...
-  hashed_url (String): [Some Hashed URL] (If you put custom_url, the hashed_url will return ""),
-  custom_url (String): [Your Custom URL] (If you didn't put any custom_url, the custom_url will return ""),

2. To find the origin URL from hashed or custom URL:
Endpoint (GET): `https://morphurl.herokuapp.com/api/v1/{url}` where `{url}` is the hashed or custom URL\
Return body (JSON):\
-  origin_url (String): https://[The origin URL]
-  hashed_url (String): [Some Hashed URL] (If you put custom_url, the hashed_url will return ""),
-  custom_url (String): [Your Custom URL] (If you didn't put any custom_url, the custom_url will return ""),

`Notes: Your origin_url should have http or https, invalid URL will return "" in hashed or custom URL`
