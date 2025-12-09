# This is simple Rust CRUD Backend using Axum:

- https://docs.rs/axum/latest/axum/
- refer this palylsit for learning rust more in depth here:
  https://www.youtube.com/playlist?list=PLDbRgZ0OOEpUkWDGqp91ODn0dk7LPBAUL

anish@LAPTOP-Q65T0SCJ:/mnt/c/Users/Anish/Documents/Anish-Coding/coding_notes/building/31-rust-backend/rust-backend$ diesel migration run
Running migration 2025-12-05-112232-0000_create_books

diesel generate <nme of  mgruiatrion>

- add authentication & use of middleware forit
- add ACCESS_TOKEN and REFRESH_TOKEN later here
- resume auth & adding users here
- add ACCESS_TOKEN, REFRESH_TOKEN and jwt (cookie managembt in frontend for it here)
- happens after register here (when user is logging in then sva ethe tokens here)

{
"token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIzNDYzYzM0OS1jOTI0LTRhZGQtOTFjZi0zM2MyODUzOGQ1MzgiLCJleHAiOjE3NjUxMDY1NzUsInVzZXJuYW1lIjoic3RldmVuIn0.Yzv3F48Mvy9Y9YlAK-LxQLKt2JECTR9jmrBi5LrUtF4",
"user": {
"created_at": "2025-12-06T11:22:54.865073",
"email": "steven@example.com",
"id": "3463c349-c924-4add-91cf-33c28538d538",
"password": "$argon2id$v=19$m=19456,t=2,p=1$ixBXHobOqPSYpmiHkypS4A$ptrjJ8q8imcTVqV0hzEBDf/ZrUR6CagwMvKI4svzvX4",
"username": "steven"
}
}

crednetuials:
{
"username": "steven",
"email": "steven@example.com",
"password": "strongpassword123"
}

{
"token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIzNDYzYzM0OS1jOTI0LTRhZGQtOTFjZi0zM2MyODUzOGQ1MzgiLCJleHAiOjE3NjUxMDczOTEsInVzZXJuYW1lIjoic3RldmVuIn0.Gy_0uzTOZWXd2BpE1wP8Jc2Mz1t27dGhUfD7jbaRXaE",
"user": {
"created_at": "2025-12-06T11:22:54.865073",
"email": "steven@example.com",
"id": "3463c349-c924-4add-91cf-33c28538d538",
"password": "$argon2id$v=19$m=19456,t=2,p=1$ixBXHobOqPSYpmiHkypS4A$ptrjJ8q8imcTVqV0hzEBDf/ZrUR6CagwMvKI4svzvX4",
"username": "steven"
}
}

# TODO:

- add REFRESH TOKEN also (right now only an ACCESS TOKEN with limited ttl)
- modularise into /routes and /controllers
- todo add an api to call gemini & create a author name and book (some axios alternate to call additonal apis here)
