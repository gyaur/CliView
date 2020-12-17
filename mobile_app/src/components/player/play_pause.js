import React, { useState } from 'react';
import { View, TouchableOpacity, StyleSheet, Text } from 'react-native';
import { AntDesign, FontAwesome } from '@expo/vector-icons';
import jsonServer from '../../api/jsonServer'
import axios from 'axios';


class PlayPause extends React.Component {

	constructor() {
		super();
		this.state = { playing: false }
	}

	playReq() {
		const res = axios.post(this.props.IP + '/play ', {})
			.then(function (response) {
				console.log(response);
			})
			.catch(function (error) {
				console.log(error);
				return ({ status: 400 })
			})
		this.setState({ playing: true });

		return ({ status: 200 })
	}


	pauseReq() {
		const res = axios.post(this.props.IP + '/pause ', {})
			.then(function (response) {
				console.log(response);
			})
			.catch(function (error) {
				console.log(error);
			})
		this.setState({ playing: true });
		return ({ status: 200 })

	}

	render() {
		return (
			<View>

				<TouchableOpacity style={{
					alignItems: "center",
					flex: 1,
					padding: 15,
					borderRadius: 80
				}}
					onPress={
						(this.state.playing === false) ?
							() => {
								this.playReq();
								this.setState({playing: true});
							}
							:
							() => {
								this.pauseReq();
								this.setState({playing: false});
							}
					}>
					{this.state.playing ?
						<FontAwesome name="pause-circle" size={60} color="#c8d6e5" /> :
						<AntDesign name="play" size={55} color="#c8d6e5" />
					}
				</TouchableOpacity>

			</View >
		)
	}

}


export default PlayPause;

