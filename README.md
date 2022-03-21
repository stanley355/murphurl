# Morph URL
Free URL shortener API to create perfect URLs for your business. MorphURL helps you create and share branded links with custom domains at scale. ✓ Check it out!


## How to
This API will shortenized your URL and return the shortened (hashed) or customized URL, you can fire the API directly through REST server or even shortenized a lot of URL in one go using Excel Uploads! (See API no 4) 

The current development is used for backend only, the next README update will include redirection functionality!


## APIs
### 1. Shortenize Single URL: `https://morphurl.herokuapp.com/api/v1` (POST)
```
Request (all required): {
  origin_url (String): https://... ,
  hashed_url (String): "",
  custom_url (String): "any custom string", (If you don't want to have any custom URL just put "" )
}

* Will return response with similar body
```

### 2. Find the origin of the shortenized URL: `https://morphurl.herokuapp.com/api/v1/{hashed_url}` (GET)
The `{hashed_url}` is the hashed_url object keys from the `/api/v1` response \
*Will return response with its origin_url + customized_url (if exist)

### 3. Shortenized an array of URL: `https://morphurl.herokuapp.com/api/v2` (POST)
```
Request: {
  shorturl_list (Array of String): [
    {
      origin_url (String): https://... ,
      hashed_url (String): "",
      custom_url (String): "Any custom string", 
    },
    {
      origin_url (String): https://... ,
      hashed_url (String): "",
      custom_url (String): "Any custom string", 
    },
    {
      origin_url (String): https://... ,
      hashed_url (String): "",
      custom_url (String): "Any custom string", 
    },
  ]
}

* Will return an Array of Objects with origin, hashed, and customized URL
```
