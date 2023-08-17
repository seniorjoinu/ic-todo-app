import './App.css';
import { createActor } from 'declarations/todo_app_backend';
import { useState, useEffect } from 'react';

const backend = createActor(process.env.REACT_APP_CANISTER_ID_TODO_APP_BACKEND);

function Element(props) {
  const [blocked, setBlocked] = useState(false);
  const [title, setTitle] = useState(props.title);

  useEffect(() => {
    setTitle(props.title);
  }, [props.title]);

  const handleCheck = () => {
    if (blocked) {
      return;
    }

    setBlocked(true);

    let newStatus;
    if (props.status.hasOwnProperty("Done")) {
      newStatus = { Todo: null };
    } else {
      newStatus = { Done: null };
    }

    backend
      .update_element_at(props.idx, { title: props.title, status: newStatus })
      .then(res => {
        if (!res.hasOwnProperty("Ok")) {
          throw new Error("Index out of bounds");
        }

        console.log("Successful element status change");
        setBlocked(false);

        props.fetchElems();
      });
  };

  const handleDelete = () => {
    if (blocked) {
      return;
    }

    setBlocked(true);

    backend.remove_element_at(props.idx).then(res => {
      if (!res.hasOwnProperty("Ok")) {
        throw new Error("Index out of bounds");
      }

      console.log("Successful element deletion");

      setBlocked(false);

      props.fetchElems();
    })
  };

  const handleChange = (event) => {
    setTitle(event.target.value);
  };

  const handleKeyPressed = (event) => {
    if (event.key !== 'Enter') {
      return;
    }

    if (blocked) {
      return;
    }

    setBlocked(true);

    const elem = { title, status: props.status };

    backend.update_element_at(props.idx, elem).then(res => {
      if (!res.hasOwnProperty("Ok")) {
        throw new Error("Index out of bounds");
      }

      console.log("Successful element update");

      setBlocked(false);

      props.deactivate();
      props.fetchElems();
    })
  }

  let text;
  if (props.isActive) {
    text = <input className='text' disabled={blocked} type={text} value={title} onKeyDown={handleKeyPressed} onChange={handleChange} />
  } else {
    text = <span className='text' onClick={props.onClick}>{title}</span>
  }

  return (
    <div className="Element">
      <input className='checkbox' type="checkbox" disabled={blocked} checked={props.status.hasOwnProperty("Done")} onClick={handleCheck} />
      {text}
      <button disabled={blocked} onClick={handleDelete}>-</button>
    </div>
  );
}

function CreateElementInput(props) {
  const [title, setTitle] = useState("");
  const [blocked, setBlocked] = useState(false);

  const handleChange = (event) => {
    setTitle(event.target.value);
  }

  const handleAdd = () => {
    if (blocked) {
      return;
    }

    setBlocked(true);

    const elem = { title, status: { Todo: null } };

    backend.add_element_at(props.len, elem).then(res => {
      if (!res.hasOwnProperty("Ok")) {
        throw new Error("Index out of bounds");
      }

      console.log("Successful adding of new element");

      setBlocked(false);
      setTitle("");

      props.fetchElems();
    });
  }

  const handleKeyPressed = (event) => {
    if (event.key !== 'Enter') {
      return;
    }

    handleAdd();
  }

  return (
    <div className='CreateElementInput'>
      <input placeholder='Добавить задачу' disabled={blocked} onKeyDown={handleKeyPressed} type="text" value={title} onChange={handleChange} />
      <button onClick={handleAdd} disabled={blocked}>+</button>
    </div>
  );
}

function App() {
  const [elements, setElements] = useState([]);
  const [activeElem, setActiveElem] = useState(null);

  const fetchElems = () => {
    backend.list_all().then(elems => {
      console.log("Fetched elements:", elems);

      setElements(elems);
    });
  };

  useEffect(fetchElems, []);

  const handleElemClick = (event, idx) => {
    event.stopPropagation();

    setActiveElem(idx);
  }

  const handleDeactivate = () => {
    setActiveElem(null);
  }

  const elems = elements.map((it, idx) => <Element key={idx} deactivate={handleDeactivate} onClick={ev => handleElemClick(ev, idx)} isActive={idx == activeElem} {...it} idx={idx} fetchElems={fetchElems} />);

  return (
    <div className="App">
      <h2>Список дел</h2>
      <CreateElementInput len={elements.length} fetchElems={fetchElems} />
      <div className='elements'>{elems}</div>
    </div>
  );
}

export default App;
