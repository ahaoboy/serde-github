# WebhookCodeScanningAlertReopened

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** |  | 
**alert** | Option<[**models::WebhookCodeScanningAlertReopenedAlert**](webhook_code_scanning_alert_reopened_alert.md)> |  | 
**commit_oid** | Option<**String**> | The commit SHA of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty. | 
**enterprise** | Option<[**models::EnterpriseWebhooks**](enterprise-webhooks.md)> |  | [optional]
**installation** | Option<[**models::SimpleInstallation**](simple-installation.md)> |  | [optional]
**organization** | Option<[**models::OrganizationSimpleWebhooks**](organization-simple-webhooks.md)> |  | [optional]
**r#ref** | Option<**String**> | The Git reference of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty. | 
**repository** | [**models::RepositoryWebhooks**](repository-webhooks.md) |  | 
**sender** | [**models::SimpleUserWebhooks**](simple-user-webhooks.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


