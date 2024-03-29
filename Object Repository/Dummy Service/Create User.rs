<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Create a New User</description>
   <name>Create User</name>
   <tag></tag>
   <elementGuidId>f2058a41-ff13-4124-a3f1-6665a2a719a6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;first_name\&quot;: \&quot;John X\&quot;,\n    \&quot;last_name\&quot; : \&quot;Rocket\&quot;,\n\t\&quot;gender\&quot;: \&quot;male\&quot;,\n    \&quot;dob\&quot;: \&quot;1962-08-12\&quot;,\n    \&quot;email\&quot;: \&quot;someone+77889116465216@domain.com\&quot;,\n    \&quot;phone\&quot;: \&quot;+657765477\&quot;,\n    \&quot;website\&quot;: \&quot;https://bit.ly/IqT6zt\&quot;,\n    \&quot;address\&quot;: \&quot;Platform 3/4 end of rainbow street\&quot;,\n    \&quot;status\&quot;: \&quot;active\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://gorest.co.in/public-api/users?_format=json&amp;access-token=Fz2YsxVmP8H35Y6CsTycMToZ0ntUQMJlLNlG</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
