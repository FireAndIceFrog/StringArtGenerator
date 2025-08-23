import { useSelector, useDispatch } from "react-redux";
import { setSettings } from "../../../shared/redux/stringArtSlice";
import type { StringArtConfig } from "../../../shared/interfaces/stringArtConfig";
import { useTranslation } from "react-i18next";
import { useMemo } from "react";

const minMaxVals: Record<
  keyof Omit<StringArtConfig, "preserve_eyes" | "preserve_negative_space">,
  [number, number, number]
> = {
  //min, max, step
  image_size: [500, 2000, 100],
  line_darkness: [25, 300, 5],
  max_lines: [800, 5000, 100],
  min_improvement_score: [0, 100, 1],
  negative_space_penalty: [0, 100, 1],
  negative_space_threshold: [0, 100, 1],
  num_nails: [360, 1440, 360 / 4],
  progress_frequency: [200, 500, 50],
};

const Slider = ({
  index,
  value,
  settings,
}: {
  index: keyof StringArtConfig;
  value: string | number;
  settings: StringArtConfig;
}) => {
  const i18next = useTranslation();

  const titles = useMemo(
    () => ({
      num_nails: i18next.t("Number of Nails"),
      image_size: i18next.t("Image Size"),
      preserve_eyes: i18next.t("Preserve Eyes"),
      preserve_negative_space: i18next.t("Preserve Negative Space"),
      negative_space_penalty: i18next.t("Negative Space Penalty"),
      negative_space_threshold: i18next.t("Negative Space Threshold"),
      max_lines: i18next.t("Max Lines"),
      line_darkness: i18next.t("Line Darkness"),
      min_improvement_score: i18next.t("Min Improvement Score"),
      progress_frequency: i18next.t("Progress Frequency"),
    }),
    [i18next]
  );

  const dispatch = useDispatch();
  const dontRender =
    !index || !Object.keys(minMaxVals).includes(index as string);
  const [min, max, step] = dontRender
    ? [0, 0, 0]
    : minMaxVals[index as keyof typeof minMaxVals];

  if (dontRender) return null;
  return (
    <div key={index}>
      <input
        value={value}
        type="range"
        id={String(`slider-${index}`)}
        name={titles[index]}
        min={min}
        max={max}
        step={step}
        onChange={({ target }) => {
          dispatch(setSettings({ ...settings, [index]: Number(target.value) }));
        }}
      />
      <label htmlFor={`slider-${index}`}>
        {titles[index]} ({value})
      </label>
    </div>
  );
};

const StringArtConfigSection = () => {
  const settings = useSelector(
    (state: { stringArt: { settings: StringArtConfig } }) =>
      state.stringArt.settings
  );
  const i18next = useTranslation();
  if (!settings) {
    return null;
  }
  return (
    <section>
      <h2>{i18next.t("String Art Configuration")}</h2>
      {(Object.keys(settings) as (keyof StringArtConfig)[]).map((key) => (
        <Slider
          key={key}
          index={key}
          value={settings[key] as unknown as string | number}
          settings={settings}
        />
      ))}
    </section>
  );
};

export default StringArtConfigSection;
