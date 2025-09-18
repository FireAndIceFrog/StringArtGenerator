# Frontend Refactoring with Material-UI Stepper

## Objective
Refactor the frontend to use Material-UI (MUI) components, specifically implementing a stepper to guide the user through the string art generation process. This will improve the user experience and make the application more extensible for future image processing steps.

## Plan

### 1. Install Material-UI Dependencies
The first step is to add the necessary MUI packages to the project.
```bash
npm install @mui/material @emotion/react @emotion/styled
```

### 2. Create a Stepper Component
A new component, `StepperScreen.tsx`, will be created to house the MUI Stepper and manage the application's flow. This component will be responsible for:
- Defining the steps: 'Upload Image' and 'Render String Art'.
- Managing the active step.
- Rendering the content for the current step.

### 3. Refactor `App.tsx`
The main `App.tsx` file will be updated to render the `StepperScreen.tsx` component instead of the `RenderImageScreen`. This will make the stepper the main UI component.

### 4. Adapt Existing Screens
The existing screens will be refactored to be used as steps within the new stepper component:
- **Step 1: Upload Image**: The content of `UploadScreen.tsx` will be used for the first step. The `handleNext` function of the stepper will be called once an image has been successfully uploaded.
- **Step 2: Render String Art**: The content of `RenderImageScreen.tsx` will be used for the second step. The `UploadScreen` component will be removed from `RenderImageScreen` to avoid duplication.

### 5. State Management
The stepper's state will be managed within the `StepperScreen.tsx` component. The Redux store will continue to be used for managing the application's global state, such as the image data and generation progress.

## File Structure Changes
- **Create**: `string-art-demo/src/features/StepperScreen.tsx`
- **Modify**: `string-art-demo/src/App.tsx`
- **Modify**: `string-art-demo/src/features/1Upload/UploadScreen.tsx`
- **Modify**: `string-art-demo/src/features/3RenderImage/RenderImageScreen.tsx`
