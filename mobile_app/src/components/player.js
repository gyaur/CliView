import React, { useState } from 'react';
import { View, TouchableOpacity, StyleSheet } from 'react-native';
import { AntDesign, FontAwesome } from '@expo/vector-icons';
import jsonServer from '../api/jsonServer'

const Player = () => {

	const [playing, setPlaying] = useState(false);
	return (
		<View style={styles.playerStyle}>
			<TouchableOpacity
				style={styles.backwardButton}
				onPress={() => {
					jsonServer.post('/seek ', { seek: -30 })
						.then(function (response) {
							console.log(response);
						})
						.catch(function (error) {
							console.log(error);
						})
					console.log('30 sec backwords')
				}}
			>
				<AntDesign name="banckward" size={24} color="#c8d6e5" />
			</TouchableOpacity>


			<TouchableOpacity style={styles.playButtonStyle}
				onPress={() => {
					if (playing == false) {
						jsonServer.post('/play ', {})
							.then(function (response) {
								console.log(response);
							})
							.catch(function (error) {
								console.log(error);
							})
						setPlaying(true);

					}
					else {
						jsonServer.post('/pause ', {})
							.then(function (response) {
								console.log(response);
							})
							.catch(function (error) {
								console.log(error);
							})
						setPlaying(false);
					}
				}}>
				{playing ?
					<FontAwesome name="pause-circle" size={60} color="#c8d6e5" /> :
					<AntDesign name="play" size={55} color="#c8d6e5" />
				}
			</TouchableOpacity>


			<TouchableOpacity style={styles.forwardButton}
				onPress={() => {
					jsonServer.post('/seek', { seek: 30 })
						.then(function (response) {
							console.log(response);
						})
						.catch(function (error) {
							console.log(error);
						})
					//console.log('30 sec forward') 
				}}>
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