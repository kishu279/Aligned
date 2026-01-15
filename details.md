# Backend Schema Design & Details

This document outlines the backend structure, database schema, and detailed API specifications for the "Aligned" app.

## 1. Database Schema

We recommend a Document Store (like Appwrite, Firebase, or MongoDB) for flexibility, but this schema is compatible with SQL.

### 1.1. Users Collection (`users`)
Stores core authentication and system-level data.

| Field | Type | Description |
| :--- | :--- | :--- |
| `id` | String (UUID) | Unique User ID |
| `email` | String | User email |
| `phone` | String | Phone number (Primary Auth) |
| `isProfileComplete` | Boolean | **TRUE only if user has 6 images + prompts** |
| `createdAt` | DateTime | Account creation time |
| `lastActive` | DateTime | Timestamp for "Active now" status |
| `preferences` | JSON | Dating preferences (see below) |

**Preferences JSON Structure:**
```json
{
  "ageRange": { "min": 18, "max": 35 },
  "distanceMax": 50, // in km or miles
  "genderPreference": ["Women"], // ["Men"], ["Everyone"]
  "ethnicityPreference": [], // Empty = open to all
  "religionPreference": []
}
```

### 1.2. Profiles Collection (`profiles`)
Detailed public-facing user profile. **One-to-One with Users**.

| Field | Type | Description |
| :--- | :--- | :--- |
| `userId` | String (Ref) | Link to `users` collection |
| `name` | String | Display name |
| `birthdate` | Date | Used to calculate `age` dynamically |
| `bio` | String | Short bio |
| `pronouns` | String | e.g., "she/her" |
| `gender` | String | e.g., "Woman", "Man", "Non-binary" |
| `sexuality` | String | e.g., "Straight", "Gay", "Bisexual" |
| `height` | Integer | Height in cm (convert to ft/in on client) |
| `location` | GeoPoint | { lat: number, lng: number } |
| `job` | String | e.g., "Actress" |
| `company` | String | (Optional) |
| `school` | String | (Optional) |
| `ethnicity` | String | e.g., "Latina", "White" |
| `politics` | String | e.g., "Moderate", "Liberal" |
| `religion` | String | e.g., "Christian", "Agnostic" |
| `relationshipType` | String | e.g., "Monogamy" |
| `datingIntention`| String | e.g., "Long-term relationship" |
| `drinks` | String | e.g., "Socially", "No" |
| `smokes` | String | e.g., "No", "Yes" |
| `imageCount` | Integer | **MUST BE >= 6 for profile to be visible** |

### 1.3. User Images (`user_images`)
Stores profile photos. **Constraint: Minimum 6 images required for a complete profile.**

| Field | Type | Description |
| :--- | :--- | :--- |
| `id` | String (UUID) | Unique Image ID |
| `userId` | String (Ref) | Owner |
| `url` | String | CDN/Storage URL |
| `order` | Integer | 0 to 5 (Slots 1-6) |
| `caption` | String | (Optional) |

### 1.4. User Prompts (`user_prompts`)
Q&A prompts. Typically 3 per user.

| Field | Type | Description |
| :--- | :--- | :--- |
| `id` | String (UUID) | Unique Prompt ID |
| `userId` | String (Ref) | Owner |
| `questionId` | String | Reference to a static list of questions |
| `question` | String | Cached question text |
| `answer` | String | User's answer |
| `order` | Integer | Display order (0-2) |

### 1.5. Matches (`matches`)
Created when two users like each other.

| Field | Type | Description |
| :--- | :--- | :--- |
| `id` | String (UUID) | Unique Match ID |
| `users` | Array[String] | IDs of the two users |
| `createdAt` | DateTime | Timestamp |
| `lastMessage` | String | Preview of last message |
| `lastMessageAt` | DateTime | Sorting timestamp |

---

## 2. API Endpoints & Contracts

### 2.1. Authentication

#### `POST /auth/phone/login`
Initiates 2FA login.
*   **Request Body:**
    ```json
    {
      "phone": "+15550109999"
    }
    ```
*   **Response:**
    ```json
    {
      "message": "OTP sent",
      "verificationId": "xyz_123_temp_id"
    }
    ```

#### `POST /auth/phone/verify`
Verifies OTP and logs in/creates user.
*   **Request Body:**
    ```json
    {
      "verificationId": "xyz_123_temp_id",
      "code": "123456"
    }
    ```
*   **Response:**
    ```json
    {
      "token": "jwt_access_token_here",
      "user": {
        "id": "user_123",
        "isProfileComplete": false,
        "isNewUser": true
      }
    }
    ```

---

### 2.2. User Profile Management

#### `GET /profile/me`
Get current user's full details.
*   **Response:**
    ```json
    {
      "id": "user_123",
      "name": "Sarah",
      "images": [
        { "id": "img_1", "url": "https://...", "order": 0 },
        { "id": "img_2", "url": "https://...", "order": 1 }
        // ... need 6 items to be "complete"
      ],
      "prompts": [
        { "id": "p_1", "question": "My simple pleasure...", "answer": "Coffee", "order": 0 }
      ],
      "details": {
        "height": 170,
        "job": "Desinger",
        // ... all profile fields
      }
    }
    ```

#### `POST /profile`
Update profile fields.
*   **Request Body:**
    ```json
    {
      "name": "Sarah",
      "bio": "Love hiking and coffee.",
      "job": "Product Designer",
      "height": 170
      // ... any other field from 1.2
    }
    ```
*   **Response:**
    ```json
    {
      "status": "success",
      "data": { ...updated_profile_object }
    }
    ```

#### `DELETE /profile`
Delete the current user's account and all associated data.
*   **Response:**
    ```json
    {
      "status": "success",
      "message": "Account deleted successfully"
    }
    ```

#### `POST /profile/images`
Upload a new image. **Backend Validation: Ensure user doesn't exceed max, but enforce minimum 6 check on "Complete Profile" action.**
*   **Request Body:** (Multipart/Form-Data)
    *   `file`: (Binary Image Data)
    *   `order`: 0
*   **Response:**
    ```json
    {
      "id": "img_999",
      "url": "https://bucket.url/img_999.jpg",
      "order": 0
    }
    ```

#### `POST /profile/finalize`
Called when user attempts to "Go Live". **Strictly checks for 6 images.**
*   **Request Body:** `{}` (Empty)
*   **Response (Success):**
    ```json
    { "success": true, "isProfileComplete": true }
    ```
*   **Response (Error):**
    ```json
    {
      "error": "INCOMPLETE_PROFILE",
      "message": "You must upload at least 6 photos to complete your profile."
    }
    ```

---

### 2.3. Feed & Discovery

#### `GET /feed`
Get potential matches to browse. Algorithm: `(Preferences Match) AND (NOT Seen) AND (Active Recently)`.
*   **Query Params:** `?limit=10&page=1`
*   **Response:**
    ```json
    {
      "profiles": [
        {
          "id": "user_456",
          "name": "John",
          "age": 28,
          "images": [ ...6_images... ],
          "prompts": [ ...3_prompts... ],
          "details": { ... }
        },
        // ... more profiles
      ]
    }
    ```

---

### 2.4. Interactions

#### `POST /interact`
Handle Like (Heart Click) or Pass (Cross Click).
*   **Request Body (Like):**
    ```json
    {
      "targetUserId": "user_456",
      "action": "LIKE", // or "PASS" (Cross Click)
      "context": {
        "type": "IMAGE", // or "PROMPT"
        "id": "img_id_that_was_liked"
      },
      "comment": "Love this hiking spot!" // Optional comment on like
    }
    ```
*   **Response (If It's a MATCH):**
    ```json
    {
      "status": "MATCH",
      "matchId": "match_789",
      "matchData": {
        "user": { "id": "user_456", "name": "John", "avatar": "..." }
      }
    }
    ```
*   **Response (Standard):**
    ```json
    { "status": "SENT" }
    ```

---

### 2.5. Messaging

#### `GET /matches`
List all conversation threads.
*   **Response:**
    ```json
    [
      {
        "id": "match_789",
        "withUser": {
          "id": "user_456",
          "name": "John",
          "avatar": "https://..."
        },
        "lastMessage": {
          "text": "Hey! How's it going?",
          "createdAt": "2024-01-12T10:00:00Z",
          "isRead": false
        }
      }
    ]
    ```

#### `GET /matches/:id/messages`
Get chat history.
*   **Query Params:** `?cursor=last_msg_id&limit=50`
*   **Response:**
    ```json
    {
      "messages": [
        {
          "id": "msg_1",
          "senderId": "user_123", // Me
          "text": "Hey!",
          "createdAt": "..."
        },
        {
          "id": "msg_2",
          "senderId": "user_456", // Them
          "text": "Hi sarah!",
          "createdAt": "..."
        }
      ]
    }
    ```

#### `POST /matches/:id/messages`
Send a text.
*   **Request Body:**
    ```json
    {
      "text": "Are you free this weekend?"
    }
    ```
*   **Response:**
    ```json
    {
      "id": "msg_3",
      "text": "Are you free this weekend?",
      "createdAt": "...",
      "status": "sent"
    }
    ```
