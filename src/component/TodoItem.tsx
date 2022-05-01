const TodoItem = () => {
  return (
    <li>
      <div className="view">
        <input type="checkbox" className="toggle" />
        <label>some text</label>
        <button className="destroy"></button>
      </div>
    </li>
  )
}
export default TodoItem
