import React from 'react';
import { View, TouchableOpacity, StyleSheet } from 'react-native';
import Slider from '@react-native-community/slider';
import { FontAwesome } from '@expo/vector-icons';

const VolumeSlide = () => {

	return (
		<View style={styles.slideContainerStyle}>
			<TouchableOpacity style={styles.iconStyle}>
				<FontAwesome name="volume-up" size={24} color="#c8d6e5" />
			</TouchableOpacity>

			<Slider
				style={styles.slideStyle}
				minimumValue={0}
				maximumValue={1}
				minimumTrackTintColor="#c8d6e5"
				maximumTrackTintColor="#576574"
				thumbTintColor="#c8d6e5"
			/>
			

		</View>
	)
};

const styles = StyleSheet.create({
	slideContainerStyle: {
		display: "flex",
		flexDirection: "row",
		justifyContent: "center",
		alignItems: "center",
		width: "90%"
	},
	slideStyle: {
		flexGrow:1,
		width: 250,
		height: 40,
	},
	iconStyle: {
		flexGrow:1,
		marginHorizontal: 10,
		padding: 10,
	}
});

export default VolumeSlide;