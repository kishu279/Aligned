# Aligned Backend API Documentation

Base URL: `http://localhost:8080`

## Health Check Endpoints (No Auth Required)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/test` | Health check endpoint |
| GET | `/health` | Health check for load balancers |

---

## Protected Routes (Authentication Required)

All routes under `/api/v1` require Firebase authentication via Bearer token in the `Authorization` header.

```
Authorization: Bearer <firebase_id_token>
```

---

### User Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/test` | Test route in protected scope |
| POST | `/api/v1/user/create` | Create a new user |
| POST | `/api/v1/user/check` | Check if user exists |
| POST | `/api/v1/user/get` | Get user details |
| POST | `/api/v1/user/preferences` | Update user preferences |

---

### Profile Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/profile/me` | Get current authenticated user's profile |
| POST | `/api/v1/profile` | Update profile fields (name, bio, etc.) |
| POST | `/api/v1/profile/finalize` | Finalize profile (sets "is_profile_complete" after ensuring 6 images) |
| DELETE | `/api/v1/profile` | Delete user account permanently |

---

### File Storage (R2 Signed URLs)

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/files/upload-url` | Get a presigned URL for uploading files to R2 storage |
| POST | `/api/v1/files/view` | View profile images |
| POST | `/api/v1/files/download-url` | Get a presigned URL for downloading files from R2 storage |

---

### Feed & Discovery

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/feed` | Get recommended profiles to swipe on |

---

### Interactions

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/interact` | Handle Like (Heart) or Pass (Cross) interactions |

---

### Matches & Messaging

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/matches` | Get list of all matches (conversations) |
| GET | `/api/v1/matches/{id}/messages` | Get chat history for a specific match |
| POST | `/api/v1/matches/{id}/messages` | Send a new message to a match |

---

### Prompts

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/prompts` | Get all prompts |
| POST | `/api/v1/prompts` | Create a new prompt |
| PUT | `/api/v1/prompts/{order}` | Update a prompt by order |
| DELETE | `/api/v1/prompts/{order}` | Delete a prompt by order |

---

## Example API Calls

### Create User
```bash
curl -X POST http://localhost:8080/api/v1/user/create \
  -H "Authorization: Bearer <firebase_token>" \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "email": "john@example.com"}'
```

### Get Profile
```bash
curl -X GET http://localhost:8080/api/v1/profile/me \
  -H "Authorization: Bearer <firebase_token>"
```

### Get Upload URL
```bash
curl -X POST http://localhost:8080/api/v1/files/upload-url \
  -H "Authorization: Bearer <firebase_token>" \
  -H "Content-Type: application/json" \
  -d '{"filename": "profile_image.jpg", "content_type": "image/jpeg"}'
```

### Get Download URL
```bash
curl -X POST http://localhost:8080/api/v1/files/download-url \
  -H "Authorization: Bearer <firebase_token>" \
  -H "Content-Type: application/json" \
  -d '{"key": "path/to/file.jpg"}'
```

### Interact with Profile
```bash
curl -X POST http://localhost:8080/api/v1/interact \
  -H "Authorization: Bearer <firebase_token>" \
  -H "Content-Type: application/json" \
  -d '{"target_user_id": "user_uuid", "action": "like"}'
```

### Get Matches
```bash
curl -X GET http://localhost:8080/api/v1/matches \
  -H "Authorization: Bearer <firebase_token>"
```

### Send Message
```bash
curl -X POST http://localhost:8080/api/v1/matches/{match_id}/messages \
  -H "Authorization: Bearer <firebase_token>" \
  -H "Content-Type: application/json" \
  -d '{"content": "Hello!"}'
```
