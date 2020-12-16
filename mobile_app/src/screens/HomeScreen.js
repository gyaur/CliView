import React, { useState, useEffect } from 'react';
import { View, Button, TextInput, StyleSheet, Text } from 'react-native';
import SearchBar from '../components/searchBar';
import Player from '../components/player';
import VolumeSlide from '../components/volumeSlide';
import StaticServer from 'react-native-static-server';
import axios from 'axios'

const HomeScreen = () => {

  const [link, setLink] = useState('');
  const [IP, setIP] = useState('http://localhost:5000');


  useEffect(() => {
    let server = new StaticServer(8080);
    console.log(server)
    //server.start().then(url => {
      //this.setState({ url });
      //console.log("Serving at URL", url);
    //});
  }, []);

  return (
    <View>
      <View style={styles.logo}>
        <Text>Logo</Text>
      </View>

      <SearchBar
        link={link}
        onLinkChange={newLink => setLink(newLink)}
        onLinkSubmit={() => {


          axios.post(IP + '/stream', { url: link })
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
            axios.post(IP + '/stream', { "url": link })
              .then(function (response) {
                console.log(response);
              })
              .catch(function (error) {
                console.log(error);
              })

          }
          }
        />
        <Text style={{ marginHorizontal: 10, color: '#c8d6e5' }}> Or</Text>
        <Button
          title="Add to queue"
          color="#576574"
          onPress={() => {
            //casting function here 
            axios.post(IP + '/queue ', { url: link })
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

      <Player newIP={IP} />

      <View style={styles.slidecontainer}>
        <VolumeSlide newIP={IP} />
      </View>

      <View style={styles.IPbackgroundStyle}>
        <Button
          style={{ marginHorizontal: 15 }}
          title="Change IP"
          color="#576574"
        />
        <TextInput
          autoCapitalize="none"
          autoCorrect={false}
          placeholder="IP"
          style={styles.IPinputStyle}
          value={IP}
          onChangeText={newIP => setIP(newIP)}
        />
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
  IPbackgroundStyle: {
    backgroundColor: '#F0EEEE',
    height: 40,
    borderRadius: 7,
    marginTop: 60,
    marginHorizontal: 15,
    flexDirection: "row"
  },
  IPinputStyle: {
    flex: 1,
    fontSize: 14,
    marginHorizontal: 25
  }
});

export default HomeScreen;