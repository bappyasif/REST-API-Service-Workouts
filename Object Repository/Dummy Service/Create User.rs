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
  &quot;text&quot;: &quot;{\n  \&quot;_meta\&quot;:{\n    \&quot;success\&quot;:false,\n    \&quot;code\&quot;:422,\n    \&quot;message\&quot;:\&quot;Data validation failed. Please check the response body for detailed error messages.\&quot;,\n    \&quot;rateLimit\&quot;:{\n      \&quot;limit\&quot;:30,\n      \&quot;remaining\&quot;:29,\n      \&quot;reset\&quot;:2\n    }\n  },\n  \&quot;result\&quot;:[\n    {\n      \&quot;field\&quot;:\&quot;email@email.com\&quot;,\n    },\n    {\n      \&quot;field\&quot;:\&quot;first_name\&quot;,\n    },\n    {\n      \&quot;field\&quot;:\&quot;last_name\&quot;,\n    },\n    {\n      \&quot;field\&quot;:\&quot;male\&quot;,\n    }]\n}\n&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
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
