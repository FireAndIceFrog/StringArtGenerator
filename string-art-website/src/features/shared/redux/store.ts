// store.ts
import { configureStore } from '@reduxjs/toolkit';
import stringArtReducer from './stringArtSlice';

export const store = configureStore({
  reducer: {
    stringArt: stringArtReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
