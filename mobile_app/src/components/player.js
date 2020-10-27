import React from 'react';
import { View, TouchableOpacity, StyleSheet } from 'react-native';
import { AntDesign } from '@expo/vector-icons';

const Player = () => {

	return (
		<View style={styles.playerStyle}>
			<TouchableOpacity
				style={styles.backwardButton}
				onPress={() => { console.log('10 sec backwords') }}
			>
				<AntDesign name="banckward" size={24} color="#c8d6e5" />
			</TouchableOpacity>
			<TouchableOpacity style={styles.playButtonStyle}
				onPress={() => { console.log('Play') }}>
				<AntDesign name="play" size={55} color="#c8d6e5" />
			</TouchableOpacity>
			<TouchableOpacity style={styles.forwardButton}
				onPress={() => { console.log('10 sec forward') }}>
				<AntDesign name="forward" size={24} color="#c8d6e5" />
			</TouchableOpacity>


		</View>
	)
};

const styles = StyleSheet.create({
	playerStyle: {
		marginTop: 50,
		flexDirection: "row",
	},
	playButtonStyle: {
		alignItems: "center",
		flex: 1,
		padding: 15,
		borderRadius: 80

	},
	forwardButton: {
		alignItems: "center",
		paddingTop: 15,
		margin: 15,
		marginRight: 25,
		flex: 1,
		borderRadius: 50

	},
	backwardButton: {
		alignItems: "center",
		paddingTop: 15,
		margin: 15,
		marginLeft: 25,
		flex: 1,
		borderRadius: 50
	}
});

export default Player;