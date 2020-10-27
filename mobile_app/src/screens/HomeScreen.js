import React, { useState } from 'react';
import { View, Button, StyleSheet, TouchableOpacity, Text } from 'react-native';
import SearchBar from '../components/searchBar';
import Player from '../components/player';
import VolumeSlide from '../components/volumeSlide';
import { Entypo } from '@expo/vector-icons';


const HomeScreen = () => {

  const [link, setLink] = useState('');

  return (
    <View>
      <View style={styles.logo}>
        <Text>Logo</Text>
      </View>

      <SearchBar
        link={link}
        onLinkChange={newLink => setLink(newLink)}
        onLinkSubmit={() =>
          //casting function here 
          console.log("link submitted : ", link)
        }
      />

      <View style={styles.castButton}>
        <Button
          style={styles.button}
          title="Cast"
          color="#576574"
          onPress={() => {
            //casting function here 
            console.log("link submitted : ", link)
          }}
        />
      </View >

      <Player />

      <View style={styles.slidecontainer}>
        <VolumeSlide />
        <TouchableOpacity
          style={styles.fullSreenIcon}
          onPress={() => { console.log('full screen ') }} >
          <Entypo name="resize-full-screen" size={24} color="#c8d6e5" />
        </TouchableOpacity>
      </View>

    </View>
  )
};

const styles = StyleSheet.create({
  logo: {
    height: 100,
    backgroundColor: 'gray',
    marginTop: 25,
    marginHorizontal: 15,
    borderRadius: 7
  },
  castButton: {
    marginTop: 5,
    marginHorizontal: 120
  },
  slidecontainer: {
    marginTop: 50,
    marginLeft: 20,
    display: "flex",
    flexDirection: "row",
    justifyContent: "center",
    alignItems: "center",
    width: "90%"
  },
  fullSreenIcon: {
    flexGrow: 1,
    marginHorizontal: 10,
    padding: 10,
  }

});

export default HomeScreen;