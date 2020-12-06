import React, { useState } from 'react';
import { View, TouchableOpacity, StyleSheet, Text } from 'react-native';
import { AntDesign } from '@expo/vector-icons';
import jsonServer from '../../api/jsonServer'

class Forward extends React.Component {

	constructor() {
		super();
		this.state = {}
	}


	forwardreq() {
		const res = jsonServer.post('/seek ', { "ammount": 30 })
			.then(function (response) {
				console.log(response);
			})
			.catch(function (error) {
				console.log(error);
				return ({ status: 400 })

			})
		return ({ status: 200 })

	}

	render() {
		return (
			<View>
				<TouchableOpacity style={{
					alignItems: "center",
					paddingTop: 15,
					margin: 15,
					marginRight: 25,
					flex: 1,
					borderRadius: 50
				}}
					onPress={() => { this.forwardreq() }}>
					<AntDesign name="forward" size={24} color="#c8d6e5" />
				</TouchableOpacity>
			</View >
		)
	}

}

export default Forward;

