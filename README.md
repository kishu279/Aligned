# 🎬 Stackshow - Movie Discovery App

A beautiful, feature-rich movie discovery app built with React Native and Expo. Browse trending movies, search for your favorites, and create your personal watchlist!

![React Native](https://img.shields.io/badge/React_Native-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)
![Expo](https://img.shields.io/badge/Expo-000020?style=for-the-badge&logo=expo&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)
![Tailwind CSS](https://img.shields.io/badge/Tailwind_CSS-38B2AC?style=for-the-badge&logo=tailwind-css&logoColor=white)

## ✨ Features

- **🏠 Home Screen** - Discover trending and latest movies
- **🔍 Search** - Find any movie with real-time search
- **📱 Movie Details** - View comprehensive movie info (rating, budget, revenue, cast)
- **💾 Save Movies** - Build your personal watchlist
- **👤 Profile** - Track your saved movies and stats
- **📊 Trending Algorithm** - Movies you search become trending (powered by Appwrite)

## 🛠️ Tech Stack

| Technology | Purpose |
|------------|---------|
| [Expo](https://expo.dev/) | React Native framework with file-based routing |
| [NativeWind](https://www.nativewind.dev/) | Tailwind CSS for React Native |
| [TypeScript](https://www.typescriptlang.org/) | Type-safe JavaScript |
| [TMDB API](https://www.themoviedb.org/) | Movie data and posters |
| [Appwrite](https://appwrite.io/) | Backend for trending algorithm |
| [AsyncStorage](https://react-native-async-storage.github.io/async-storage/) | Local storage for saved movies |

## 🚀 Real-World Use Cases

### 1. **Personal Movie Tracker**
Use Stackshow to keep track of all the movies you want to watch. No more forgetting recommendations from friends!

### 2. **Movie Night Planner**
Planning a movie night? Browse popular movies, save options to your watchlist, and pick from your curated list when the time comes.

### 3. **Film Discovery Platform**
Discover new movies based on what's trending. The app tracks popular searches, surfacing movies that others are interested in.

### 4. **Movie Research Tool**
Before watching a movie, check its rating, budget, revenue, and production companies to make informed viewing decisions.

### 5. **Social Recommendation Engine**
The trending feature creates a community-driven recommendation system - movies that get searched more often appear in the trending section.

## 📦 Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/stackshow.git
   cd stackshow
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Set up environment variables**
   
   Create a `.env` file in the root directory:
   ```env
   EXPO_PUBLIC_MOVIE_API_KEY=your_tmdb_api_key
   EXPO_PUBLIC_TMDB_READ_TOKEN=your_tmdb_read_token
   EXPO_PUBLIC_APPWRITE_PROJECT_ID=your_appwrite_project_id
   EXPO_PUBLIC_APPWRITE_DATABASE_ID=your_appwrite_database_id
   EXPO_PUBLIC_APPWRITE_COLLECTION_ID=your_appwrite_collection_id
   ```

4. **Start the development server**
   ```bash
   npx expo start
   ```

5. **Run on your device**
   - Scan the QR code with Expo Go (Android) or Camera app (iOS)
   - Or press `a` for Android emulator / `i` for iOS simulator

## 🔧 Appwrite Setup (for Trending Feature)

1. Create an account at [cloud.appwrite.io](https://cloud.appwrite.io)
2. Create a new project
3. Create a database with a collection containing these attributes:

| Attribute | Type | Required |
|-----------|------|----------|
| `searchTerm` | String (255) | Yes |
| `movie_id` | Integer | Yes |
| `title` | String (255) | Yes |
| `count` | Integer | Yes |
| `poster_url` | URL | Yes |

4. Set collection permissions: **Any** → Create, Read, Update
5. Add your credentials to `.env`

## 📁 Project Structure

```
stackshow/
├── app/                    # Expo Router pages
│   ├── (tabs)/            # Tab-based navigation
│   │   ├── index.tsx      # Home screen
│   │   ├── search.tsx     # Search screen
│   │   ├── saved.tsx      # Saved movies
│   │   └── profile.tsx    # User profile
│   └── movies/
│       └── [id].tsx       # Movie details
├── components/            # Reusable components
│   ├── MovieCard.tsx
│   ├── SearchBar.tsx
│   └── TrendingCard.tsx
├── services/              # API & business logic
│   ├── api.ts            # TMDB API calls
│   ├── appwrite.ts       # Appwrite integration
│   ├── storage.ts        # AsyncStorage functions
│   └── useFetch.ts       # Custom fetch hook
├── constants/            # App constants
├── interfaces/           # TypeScript interfaces
└── assets/              # Images, fonts, icons
```

## 📱 Screenshots

| Home | Search | Details | Saved |
|------|--------|---------|-------|
| Trending & Latest Movies | Real-time Search | Full Movie Info | Your Watchlist |

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

## 🙏 Acknowledgments

- [TMDB](https://www.themoviedb.org/) for the movie database API
- [JavaScript Mastery](https://www.youtube.com/@javascriptmastery) for the tutorial inspiration
- [Appwrite](https://appwrite.io/) for the backend services

---

**Made with ❤️ by Subh**
