import 'react-native';
import React from 'react';
import HomeScreen from '../screens/HomeScreen';
import renderer from 'react-test-renderer';

test('HomeScreen snapshot', ()=>{
    
    const snap = renderer.create(
        <HomeScreen/>
    ).toJSON();
   
    expect(snap).toMatchSnapshot();
});