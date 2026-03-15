const STATUS_CLASS = {
  connected: 'is-connected',
  available_not_configured: 'is-available',
  planned_manual: 'is-manual',
  blocked: 'is-blocked',
  planned: 'is-planned',
  active: 'is-active',
  draft: 'is-draft',
  pending_review: 'is-pending-review'
};

function WorkspaceStatusPill({ status, label }) {
  const className = STATUS_CLASS[status] || 'is-planned';

  return <span className={`workspace-status-pill ${className}`}>{label || status}</span>;
}

export default WorkspaceStatusPill;
