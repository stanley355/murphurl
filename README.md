# Shorten URL
Powerful and Fastest Short URL creator built in Rust

## Docs
1. To create shortened URL:
POST: `https://morph.com/api/shortenurl`
Body (JSON): {
  origin_url (String): https://... (Required),
  custom_url (String): https://... (Required, if you don't want to have any custom URL just put "" ),
} 

Return (JSON): {
  origin_url (String): https://...
  hashed_url (String): [Some Hashed URL] (If you put custom_url, the hashed_url will return ""),
  custom_url (String): [Your Custom URL] (If you put didn't put any custom_url, the custom_url will return ""),
}

2. To share shortened URL:
ENDPOINT: `https://morph.com/url/{url}` (where `{url}` is your hashed or custom URL )
