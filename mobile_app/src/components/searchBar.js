import React from 'react';
import { View, TextInput, StyleSheet } from 'react-native';
import { Feather } from '@expo/vector-icons';

const SearchBar = ({ link, onLinkChange, onLinkSubmit }) => {
	return (
		<View style={styles.backgroundStyle}>
			<Feather name="search" style={styles.iconStyle} />
			<TextInput
				autoCapitalize = "none"
				autoCorrect = {false}
				placeholder="URL"
				style={styles.inputStyle}
				value={link}
				onChangeText={onLinkChange}
				onEndEditing={onLinkSubmit}
			/>
		</View>
	)
};

const styles = StyleSheet.create({
	backgroundStyle: {
		backgroundColor: '#F0EEEE',
		height: 50,
		borderRadius: 7,
		marginTop: 15,
		marginHorizontal: 15,
		flexDirection: "row"
	},
	inputStyle: {
		flex: 1,
		fontSize: 18
	},
	iconStyle: {
		fontSize: 35,
		alignSelf: 'center',
		marginHorizontal: 15
	}

});

export default SearchBar;