function WorkspaceSectionCard({ title, subtitle, children }) {
  return (
    <article className="workspace-card">
      <header className="workspace-card-header">
        <h2>{title}</h2>
        {subtitle ? <p>{subtitle}</p> : null}
      </header>
      <div className="workspace-card-body">{children}</div>
    </article>
  );
}

export default WorkspaceSectionCard;
