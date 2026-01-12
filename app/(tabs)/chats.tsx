import { Text, View } from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";

export default function Chats() {
    return (
        <View className="flex-1 bg-white">
            <SafeAreaView className="flex-1 px-5">
                <Text className="text-black text-2xl font-bold mt-10 font-nunito">Matches</Text>
                <Text className="text-gray-500 mt-4 font-nunito">Your conversations will appear here.</Text>
            </SafeAreaView>
        </View>
    );
}
