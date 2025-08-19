// stringArtSlice.ts
import { createAsyncThunk, createSlice, type PayloadAction } from '@reduxjs/toolkit';
import type { StringArtConfig } from '../interfaces/stringArtConfig';
import { generateStringArt } from '../services/StringArtService';
import type { ProgressInfo } from '../../../wasm/string_art_rust_impl';

export interface StringArtState {
  imageData: Uint8Array | null;
  imageUrl: string;
  isGenerating: boolean;
  currentPath: number[];
  nailCoords: Array<[number, number]>;
  progress: ProgressInfo | null;
  isLoading: boolean;
  error: string | null;
  settings: StringArtConfig;
}

interface StrinArtThunkProperties { imageData: Uint8Array; settings: StringArtConfig; }

const initialState: StringArtState = {
  imageData: null,
  imageUrl: '',
  isGenerating: false,
  currentPath: [],
  nailCoords: [],
  progress: null,
  isLoading: false,
  error: null,
  settings: {
    num_nails: 360,
    image_size: 500,
    extract_subject: false,
    remove_shadows: false,
    preserve_eyes: true,
    preserve_negative_space: false,
    negative_space_penalty: 5,
    negative_space_threshold: 0.5,
    max_lines: 2000,
    line_darkness: 100,
    min_improvement_score: 15,
    progress_frequency: 300,
  },
};

// Async thunk for string art generation
export const generateStringArtThunk = createAsyncThunk(
  'stringArt/generate',
  async (
    { imageData, settings }: StrinArtThunkProperties,
    {dispatch}
  ) => {
    return await generateStringArt(
      settings,
      imageData,
      // Progress and nailCoords will be handled via events and dispatched separately if needed
      (progressInfo) => {
        dispatch(appendToCurrentPath([...progressInfo.current_path]));
      },
      (nailCoords) => {
        dispatch(setNailCoords(nailCoords));
      }
    );
  }
);

// Actions for async generation will be added later via thunk
const stringArtSlice = createSlice({
  name: 'stringArt',
  initialState,
  reducers: {
    setImageData(state: StringArtState, action: PayloadAction<Uint8Array | null>) {
      state.imageData = action.payload;
    },
    setNailCoords(state: StringArtState, action: PayloadAction<Array<[number, number]>>) {
      state.nailCoords = action.payload;
    },
    appendToCurrentPath(state: StringArtState, action: PayloadAction<number[]>) {
      state.currentPath = [...state.currentPath, ...action.payload];
    },
    setImageUrl(state: StringArtState, action: PayloadAction<string>) {
      state.imageUrl = action.payload;
    },
    setIsLoading(state: StringArtState, action: PayloadAction<boolean>) {
      state.isLoading = action.payload;
    },
    setSettings(state: StringArtState, action: PayloadAction<StringArtConfig>) {
      state.settings = action.payload;
    },
    resetState(state: StringArtState) {
      state.imageData = null;
      state.imageUrl = '';
      state.isGenerating = false;
      state.currentPath = [];
      state.nailCoords = [];
      state.progress = null;
      state.error = null;
    },
  },
  extraReducers: (builder) => {
    builder
      .addCase(generateStringArtThunk.pending, (state) => {
        state.isGenerating = true;
        state.currentPath = [];
        state.progress = null;
        state.error = null;
      })
      .addCase(generateStringArtThunk.fulfilled, (state, action) => {
        // result.path and result.nailCoords from thunk payload
        const result = action.payload;
        if (result.path) {
          const pathArr: number[] = Array.isArray(result.path)
            ? result.path
            : Array.from(result.path);
          state.currentPath = [...state.currentPath, ...pathArr];
        }
        if (result.nailCoords.length > 0) {
          state.nailCoords = result.nailCoords;
        }
        state.isGenerating = false;
      })
      .addCase(generateStringArtThunk.rejected, (state, action) => {
        state.isGenerating = false;
        state.error = action.error.message || 'String art generation failed';
      });
  },
});

export const {
  setImageData,
  setImageUrl,
  setIsLoading,
  setSettings,
  resetState,
  appendToCurrentPath,
  setNailCoords,
} = stringArtSlice.actions;

export default stringArtSlice.reducer;
