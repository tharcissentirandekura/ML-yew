use yew::prelude::*;
use web_sys::HtmlSelectElement;

#[derive(Properties, PartialEq)]
pub struct LabelsProps {
    pub on_select: Callback<String>,
}

#[function_component(SelectLabels)]
pub fn labels(props: &LabelsProps) -> Html {
    let onchange = {
        let on_select = props.on_select.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            let value = select.value();
            on_select.emit(value);
        })
    };

    

    html! {
        <select  
            id="label-select"
            class="select-toggle text-gray-900 bg-white w-50 p-2.5 border-none rounded-sm shadow-md" 
            multiple=false 
            {onchange}
        >
            <option value="person">{"person"}</option>
            <option value="bicycle">{"bicycle"}</option>
            <option value="car">{"car"}</option>
            <option value="motorcycle">{"motorcycle"}</option>
            <option value="airplane">{"airplane"}</option>
            <option value="bus">{"bus"}</option>
            <option value="train">{"train"}</option>
            <option value="truck">{"truck"}</option>
            <option value="boat">{"boat"}</option>
            <option value="traffic light">{"traffic light"}</option>
            <option value="fire hydrant">{"fire hydrant"}</option>
            <option value="stop sign">{"stop sign"}</option>
            <option value="parking meter">{"parking meter"}</option>
            <option value="bench">{"bench"}</option>
            <option value="bird">{"bird"}</option>
            <option value="cat">{"cat"}</option>
            <option value="dog">{"dog"}</option>
            <option value="horse">{"horse"}</option>
            <option value="sheep">{"sheep"}</option>
            <option value="cow">{"cow"}</option>
            <option value="elephant">{"elephant"}</option>
            <option value="bear">{"bear"}</option>
            <option value="zebra">{"zebra"}</option>
            <option value="giraffe">{"giraffe"}</option>
            <option value="backpack">{"backpack"}</option>
            <option value="umbrella">{"umbrella"}</option>
            <option value="handbag">{"handbag"}</option>
            <option value="tie">{"tie"}</option>
            <option value="suitcase">{"suitcase"}</option>
            <option value="frisbee">{"frisbee"}</option>
            <option value="skis">{"skis"}</option>
            <option value="snowboard">{"snowboard"}</option>
            <option value="sports ball">{"sports ball"}</option>
            <option value="kite">{"kite"}</option>
            <option value="baseball bat">{"baseball bat"}</option>
            <option value="baseball glove">{"baseball glove"}</option>
            <option value="skateboard">{"skateboard"}</option>
            <option value="surfboard">{"surfboard"}</option>
            <option value="tennis racket">{"tennis racket"}</option>
            <option value="bottle">{"bottle"}</option>
            <option value="wine glass">{"wine glass"}</option>
            <option value="cup">{"cup"}</option>
            <option value="fork">{"fork"}</option>
            <option value="knife">{"knife"}</option>
            <option value="spoon">{"spoon"}</option>
            <option value="bowl">{"bowl"}</option>
            <option value="banana">{"banana"}</option>
            <option value="apple">{"apple"}</option>
            <option value="sandwich">{"sandwich"}</option>
            <option value="orange">{"orange"}</option>
            <option value="broccoli">{"broccoli"}</option>
            <option value="carrot">{"carrot"}</option>
            <option value="hot dog">{"hot dog"}</option>
            <option value="pizza">{"pizza"}</option>
            <option value="donut">{"donut"}</option>
            <option value="cake">{"cake"}</option>
            <option value="chair">{"chair"}</option>
            <option value="couch">{"couch"}</option>
            <option value="potted plant">{"potted plant"}</option>
            <option value="bed">{"bed"}</option>
            <option value="dining table">{"dining table"}</option>
            <option value="toilet">{"toilet"}</option>
            <option value="tv">{"tv"}</option>
            <option value="laptop">{"laptop"}</option>
            <option value="mouse">{"mouse"}</option>
            <option value="remote">{"remote"}</option>
            <option value="keyboard">{"keyboard"}</option>
            <option value="cell phone">{"cell phone"}</option>
            <option value="microwave">{"microwave"}</option>
            <option value="oven">{"oven"}</option>
            <option value="toaster">{"toaster"}</option>
            <option value="sink">{"sink"}</option>
            <option value="refrigerator">{"refrigerator"}</option>
            <option value="book">{"book"}</option>
            <option value="clock">{"clock"}</option>
            <option value="vase">{"vase"}</option>
            <option value="scissors">{"scissors"}</option>
            <option value="teddy bear">{"teddy bear"}</option>
            <option value="hair drier">{"hair drier"}</option>
            <option value="toothbrush">{"toothbrush"}</option>
            <option value="option1">{"Select a label below"}</option>
        </select>
    }
}