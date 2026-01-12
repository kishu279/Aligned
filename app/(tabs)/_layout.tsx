import { Ionicons } from "@expo/vector-icons";
import { Tabs } from "expo-router";
import { useEffect } from "react";
import { Image, ImageBackground, ImageSourcePropType, StyleSheet } from "react-native";
import Animated, {
  useAnimatedStyle,
  useSharedValue,
  withSpring,
} from "react-native-reanimated";

// Import icon background
const iconBg = require("@/assets/icons/icon-bg.png");

// Icon assets
const icons = {
  home: "home" as const,
  star: require("@/assets/icons/star.png"),
  heart: require("@/assets/icons/heart.png"),
  chat: require("@/assets/icons/chat.png"),
  profile: require("@/assets/icons/profile.png"),
};

type TabIconProps = {
  focused: boolean;
  iconName: keyof typeof icons;
};

function TabIcon({ focused, iconName }: TabIconProps) {
  const scale = useSharedValue(1);

  useEffect(() => {
    scale.value = withSpring(focused ? 1 : 0.95, {
      damping: 15,
      stiffness: 150
    });
  }, [focused]);

  const animatedStyle = useAnimatedStyle(() => ({
    transform: [{ scale: scale.value }],
  }));

  // Home uses Ionicons, others use PNG
  const isHome = iconName === "home";
  const iconSource = !isHome ? icons[iconName] : null;

  if (focused) {
    return (
      <Animated.View style={[animatedStyle, styles.tabIconWrapper]}>
        <ImageBackground
          source={iconBg}
          style={styles.iconBgContainer}
          imageStyle={styles.iconBgImage}
          resizeMode="contain"
        >
          {isHome ? (
            <Ionicons name="home" size={24} color="#000000" />
          ) : (
            <Image
              source={iconSource as ImageSourcePropType}
              style={styles.iconImage}
              resizeMode="contain"
            />
          )}
        </ImageBackground>
      </Animated.View>
    );
  }

  return (
    <Animated.View style={[animatedStyle, styles.tabIconWrapper]}>
      {isHome ? (
        <Ionicons name="home-outline" size={24} color="#525252" />
      ) : (
        <Image
          source={iconSource as ImageSourcePropType}
          style={[styles.iconImage, styles.iconUnfocused]}
          resizeMode="contain"
        />
      )}
    </Animated.View>
  );
}

const styles = StyleSheet.create({
  tabIconWrapper: {
    alignItems: "center",
    justifyContent: "center",
    width: 44,
    height: 44,
  },
  iconBgContainer: {
    width: 40,
    height: 40,
    alignItems: "center",
    justifyContent: "center",
  },
  iconBgImage: {
    width: 40,
    height: 40,
  },
  iconImage: {
    width: 22,
    height: 22,
    tintColor: "#000000",
  },
  iconUnfocused: {
    tintColor: "#525252",
  },
});

export default function TabsLayout() {
  return (
    <Tabs
      screenOptions={{
        tabBarShowLabel: false,
        tabBarItemStyle: {
          justifyContent: "center",
          alignItems: "center",
          paddingVertical: 8,
        },
        tabBarStyle: {
          backgroundColor: "#FFFFFF",
          borderRadius: 35,
          marginHorizontal: 20,
          marginBottom: 28,
          height: 56,
          position: "absolute",
          overflow: "hidden",
          borderWidth: 1,
          borderColor: "#E5E5E5",
          shadowColor: "#000",
          shadowOffset: { width: 0, height: 4 },
          shadowOpacity: 0.08,
          shadowRadius: 12,
          elevation: 5,
          paddingHorizontal: 8,
        },
      }}
    >
      <Tabs.Screen
        name="index"
        options={{
          title: "Home",
          headerShown: false,
          tabBarIcon: ({ focused }) => (
            <TabIcon focused={focused} iconName="home" />
          ),
        }}
      />

      <Tabs.Screen
        name="start"
        options={{
          title: "Start",
          headerShown: false,
          tabBarIcon: ({ focused }) => (
            <TabIcon focused={focused} iconName="star" />
          ),
        }}
      />

      <Tabs.Screen
        name="likes"
        options={{
          title: "Likes",
          headerShown: false,
          tabBarIcon: ({ focused }) => (
            <TabIcon focused={focused} iconName="heart" />
          ),
        }}
      />

      <Tabs.Screen
        name="chats"
        options={{
          title: "Chats",
          headerShown: false,
          tabBarIcon: ({ focused }) => (
            <TabIcon focused={focused} iconName="chat" />
          ),
        }}
      />

      <Tabs.Screen
        name="profile"
        options={{
          title: "Profile",
          headerShown: false,
          tabBarIcon: ({ focused }) => (
            <TabIcon focused={focused} iconName="profile" />
          ),
        }}
      />
    </Tabs>
  );
}
