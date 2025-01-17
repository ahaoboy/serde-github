# ReposUpdateOrgRulesetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the ruleset. | [optional]
**target** | Option<**String**> | The target of the ruleset  **Note**: The `push` target is in beta and is subject to change. | [optional]
**enforcement** | Option<[**models::RepositoryRuleEnforcement**](repository-rule-enforcement.md)> |  | [optional]
**bypass_actors** | Option<[**Vec<models::RepositoryRulesetBypassActor>**](repository-ruleset-bypass-actor.md)> | The actors that can bypass the rules in this ruleset | [optional]
**conditions** | Option<[**models::OrgRulesetConditions**](org-ruleset-conditions.md)> |  | [optional]
**rules** | Option<[**Vec<models::RepositoryRule>**](repository-rule.md)> | An array of rules within the ruleset. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


