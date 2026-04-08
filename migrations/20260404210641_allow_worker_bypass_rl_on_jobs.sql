-- Drop old policy
DROP POLICY IF EXISTS tenant_isolation_jobs ON jobs;

-- Create new policy that allows bypass if 'app.bypass_rls' is 'on'
CREATE POLICY tenant_isolation_jobs ON jobs
    USING (
        (current_setting('app.current_tenant', true) <> '' AND tenant_id = current_setting('app.current_tenant', true)::UUID)
        OR current_setting('app.bypass_rls', true) = 'on'
    );
