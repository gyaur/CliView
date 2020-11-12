import React, { useState } from 'react';
import { View, Button, StyleSheet, Text } from 'react-native';
import SearchBar from '../components/searchBar';
import Player from '../components/player';
import VolumeSlide from '../components/volumeSlide';
import { Entypo } from '@expo/vector-icons';
import jsonServer from '../api/jsonServer'

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
        onLinkSubmit={() => {
          jsonServer.post('/stream', { url: link })
            .then(function (response) {
              console.log(response);
            })
            .catch(function (error) {
              console.log(error);
            })
          console.log("link submitted : ", link)
        }
        }
      />

      <View style={styles.castButton}>
        <Button
          title="Cast now"
          color="#576574"
          onPress={() => {
            jsonServer.post('/stream', { url: link })
              .then(function (response) {
                console.log(response);
              })
              .catch(function (error) {
                console.log(error);
              })
            console.log("link submitted : ", link)
          }}
        />
        <Text style={{marginHorizontal: 10, color: '#c8d6e5'}}> Or</Text>
        <Button
          title="Add to queue"
          color="#576574"
          onPress={() => {
            //casting function here 
            jsonServer.post('/queue ', { url: link })
              .then(function (response) {
                console.log(response);
              })
              .catch(function (error) {
                console.log(error);
              })
            console.log("link submitted : ", link)
          }}
        />
      </View >

      <Player />

      <View style={styles.slidecontainer}>
        <VolumeSlide />
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
    marginTop: 15,
    display: "flex",
    flexDirection: "row",
    justifyContent: "center",
    alignItems: "center",
  },
  slidecontainer: {
    marginTop: 50,
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