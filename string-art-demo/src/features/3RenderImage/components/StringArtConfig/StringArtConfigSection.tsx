import { useSelector, useDispatch } from "react-redux";
import { type StringArtConfig } from "../../../shared/hooks/useStringArt";
import { setSettings } from "../../../shared/redux/stringArtSlice";

const Slider = ({
  title,
  index,
  value,
  settings,
}: {
  title: string;
  index: keyof StringArtConfig;
  value: string | number;
  settings: StringArtConfig;
}) => {
  const minMaxVals: Record<
    keyof Omit<
      StringArtConfig,
      | "extract_subject"
      | "remove_shadows"
      | "preserve_eyes"
      | "preserve_negative_space"
    >,
    [number, number, number]
  > = {
    //min, max, step
    image_size: [500, 2000, 100],
    line_darkness: [25, 300, 5],
    max_lines: [800, 5000, 100],
    min_improvement_score: [0, 100, 1],
    negative_space_penalty: [0, 100, 1],
    negative_space_threshold: [0, 100, 1],
    num_nails: [360, 1440, 360/4],
    progress_frequency: [200, 500, 50],
  };

  const dispatch = useDispatch();
  const dontRender = !index || !Object.keys(minMaxVals).includes(index as string)
  const [min, max, step] = dontRender? [0,0,0] : minMaxVals[index as keyof typeof minMaxVals];

  if (dontRender) return null;
  return (
    <div key={index}>
      <input
        value={value}
        type="range"
        id={`slider-${index}`}
        name={title}
        min={min}
        max={max}
        step={step}
        onChange={({ target }) => {
          dispatch(setSettings({ ...settings, [index]: Number(target.value) }));
        }}
      />
      <label htmlFor={`slider-${index}`}>{title} ({value})</label>
    </div>
  );
};

const StringArtConfigSection = () => {
  const settings = useSelector((state: { stringArt: { settings: StringArtConfig } }) => state.stringArt.settings);

  if (!settings) {
    return null;
  }
  return (
    <section>
      <h2>String Art Configuration</h2>
      {(Object.keys(settings) as (keyof StringArtConfig)[]).map((key) =>
        (
          <Slider
            key={key}
            index={key}
            title={key.split("_").join(" ")}
            value={settings[key] as unknown as string | number}
            settings={settings}
          />
        )
      )}
    </section>
  );
};

export default StringArtConfigSection;
