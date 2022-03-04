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
  custom_url (String): https://... , (If you don't want to have any custom URL just put "" )
}

* Will return response with similar body
```

### 2. Find the origin of the shortenized URL: `https://morphurl.herokuapp.com/api/v1/{hashed_url}` (GET)
The `{hashed_url}` is the hashed_url object keys from the `/api/v1` response \
*Will return response with its origin_url + customized_url (if exist)

### 3. Shortenized Array URL: `https://morphurl.herokuapp.com/api/v2` (POST)
```
Request: {
  url_list (Array of String): ["https://... ", "https://... ", ...]
}

* Will return an Array of Objects with origin, hashed, and customized URL
```

### 4. Shortenized URL via Excel: `https://morphurl.herokuapp.com/api/v2/excel` (POST)
Instruction: 
- Create an excel file with two columns \
![image](https://user-images.githubusercontent.com/53996155/156747848-b4e685e8-ceeb-4cb2-a6c8-fd9ffb0b959c.png)
\
- The first column is the origin URL and the second column is the customized URL (empty it if unnecessary)
- Send the excel file with Content-Type: multipart/form-data header

* Will return an Array of Objects with origin, hashed, and customized URL

