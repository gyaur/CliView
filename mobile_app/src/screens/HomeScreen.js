import React, { useState, useEffect } from 'react';
import { View, Button, TextInput, StyleSheet, Text, Image } from 'react-native';
import SearchBar from '../components/searchBar';
import Player from '../components/player';
import VolumeSlide from '../components/volumeSlide';
//import StaticServer from 'react-native-static-server';
import axios from 'axios'


const HomeScreen = () => {

  const [link, setLink] = useState('');
  const [IP, setIP] = useState('http://localhost:5000');
  const [IPInput, setIPInput] = useState(false)


  return (
    <View>
      <View>
        <Image
          style={{
            height: 150, marginHorizonta: 20, marginTop: 10,
          }}
          source={require('../assets/logo.png')}
        />
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
          title="   Cast now   "
          color="#576574"
          style={{ margin: 30 }}
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
        <Text style={{ marginHorizontal: 15, color: '#c8d6e5' }}> Or</Text>
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

      <View style={{
        flexDirection: 'row',
        marginTop: 35,
        justifyContent: "center",
        alignItems: "center",
      }}>

        <Button
          title="Skip to the next video"
          color="#576574"
          onPress={() => {  
            axios.post(IP + '/skip', {})
            .then(function (response) {
              console.log(response);
            })
            .catch(function (error) {
              console.log(error);
            })

          }} 
        />
      </View>


      <Player newIP={IP} />


      <View style={styles.slidecontainer}>
        <VolumeSlide newIP={IP} />
      </View>


      <View style={{
        marginLeft: "auto",
        marginRight: "auto",
        marginTop: 50,
        flexDirection: "row",
        backgroundColor: '#F0EEEE',
        height: 40,
        borderRadius: 7,
      }}>
        <Button
          title="Change IP"
          color="#576574"
          onPress={() => { IPInput ? setIPInput(false) : setIPInput(true) }}
        />

        <View>
          {IPInput ?
            <TextInput
              autoCapitalize="none"
              autoCorrect={false}
              placeholder="IP"
              style={styles.IPinputStyle}
              value={IP}
              onChangeText={newIP => setIP(newIP)}
            /> : null}

        </View>
      </View>

    </View>
  )
};

const styles = StyleSheet.create({

  castButton: {
    marginTop: 15,
    flexDirection: "row",
    justifyContent: "center",
    alignItems: "center",
  },
  slidecontainer: {
    marginTop: 30,
    display: "flex",
    flexDirection: "row",
    justifyContent: "center",
    alignItems: "center",
    width: "90%"
  },
  IPinputStyle: {
    flex: 1,
    fontSize: 14,
    marginHorizontal: 10
  }
});

export default HomeScreen;