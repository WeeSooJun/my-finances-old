use yew::{function_component, html, Html};

#[function_component(Table)]
pub fn table() -> Html {
    html! { 
        <table>
        <tr>
            <th>{"Date"}</th>
            <th>{"Name"}</th>
            <th>{"Category"}</th>
            <th>{"Transaction Type"}</th>
            <th>{"Amount"}</th>
        </tr>
        <tr>
            <td>{"10 Nov"}</td>
            <td>{"Cai Fan"}</td>
            <td>{"Food"}</td>
            <td>{"Paylah"}</td>
            <td>{5.00}</td>
        </tr>
        <tr>
            <td>{"10 Nov"}</td>
            <td>{"Bee Hoon"}</td>
            <td>{"Food"}</td>
            <td>{"DBS - Card"}</td>
            <td>{"4.50"}</td>
        </tr>
        </table>
    }
}