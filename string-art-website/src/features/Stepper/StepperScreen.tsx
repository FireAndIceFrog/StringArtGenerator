import * as React from 'react';
import Box from '@mui/material/Box';
import Stepper from '@mui/material/Stepper';
import Step from '@mui/material/Step';
import StepLabel from '@mui/material/StepLabel';
import Button from '@mui/material/Button';
import Typography from '@mui/material/Typography';
import UploadScreen from '../1Upload/UploadScreen';
import RenderImageScreen from '../3RenderImage/RenderImageScreen';
import { useSelector } from 'react-redux';
import { type StringArtState } from '../shared/redux/stringArtSlice';
import { useTranslation } from 'react-i18next';


export default function StepperScreen() {
  const i18next = useTranslation();
  const steps = [i18next.t('Upload Image'), i18next.t('Render String Art')];
  const [activeStep, setActiveStep] = React.useState(0);
  const { imageData } = useSelector((state: { stringArt: StringArtState }) => state.stringArt);
  const renderImageScreenRef = React.useRef<HTMLCanvasElement>(null);
  const handleNext = () => {
    setActiveStep((prevActiveStep) => prevActiveStep + 1);
  };

  const handleBack = () => {
    setActiveStep((prevActiveStep) => prevActiveStep - 1);
  };

  const handleReset = () => {
    setActiveStep(0);
  };

  const handleFinish = () => {
    if (renderImageScreenRef.current) {
      const canvas = renderImageScreenRef.current;
      const ctx = canvas.getContext('2d');

      if (ctx) {
        // Create a new canvas to draw the image with a white background
        const tempCanvas = document.createElement('canvas');
        tempCanvas.width = canvas.width;
        tempCanvas.height = canvas.height;
        const tempCtx = tempCanvas.getContext('2d');

        if (tempCtx) {
          tempCtx.fillStyle = 'white';
          tempCtx.fillRect(0, 0, tempCanvas.width, tempCanvas.height);
          tempCtx.drawImage(canvas, 0, 0);

          const image = tempCanvas.toDataURL('image/png');
          const link = document.createElement('a');
          link.href = image;
          link.download = 'string-art.png';
          document.body.appendChild(link);
          link.click();
          document.body.removeChild(link);
        }
      }
    }
    handleReset();
  };

  return (
    <Box sx={{ width: '100%', maxWidth:1000, mx: 'auto' }}> {/* Removed maxWidth and padding, kept centering */}
      <Stepper activeStep={activeStep}>
        {steps.map((label) => {
          const stepProps: { completed?: boolean } = {};
          const labelProps: {
            optional?: React.ReactNode;
          } = {};
          return (
            <Step key={label} {...stepProps}>
              <StepLabel {...labelProps}>{label}</StepLabel>
            </Step>
          );
        })}
      </Stepper>
      {activeStep === steps.length ? (
        <React.Fragment>
          <Typography sx={{ mt: 2, mb: 1 }}>
            {i18next.t("All steps completed - you're finished")}
          </Typography>
          <Box sx={{ display: 'flex', flexDirection: 'row', pt: 2 }}>
            <Box sx={{ flex: '1 1 auto' }} />
            <Button onClick={handleFinish}>{i18next.t("Download Image")}</Button>
          </Box>
        </React.Fragment>
      ) : (
        <React.Fragment>
          {activeStep === 0 && <UploadScreen onImageSelected={handleNext} />}
          {activeStep === 1 && <RenderImageScreen ref={renderImageScreenRef} />}
          <Box sx={{ display: 'flex', flexDirection: 'row', pt: 2 }}>
            <Button
              color="inherit"
              disabled={activeStep === 0}
              onClick={handleBack}
              sx={{ mr: 1 }}
            >
              {i18next.t("Back")}
            </Button>
            <Box sx={{ flex: '1 1 auto' }} />
            <Button onClick={activeStep === steps.length - 1 ? handleFinish : handleNext} disabled={!imageData && activeStep === 0}>
              {activeStep === steps.length - 1 ? i18next.t("Finish") : i18next.t("Next")}
            </Button>
          </Box>
        </React.Fragment>
      )}
    </Box>
  );
}
