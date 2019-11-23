<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Update Authors Details Request</description>
   <name>Update Authors Details Request</name>
   <tag></tag>
   <elementGuidId>d0242721-fb44-4d9f-a7a3-ffee6e6babd0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;ID\&quot;: \&quot;${author_id}\&quot;,\n  \&quot;IDBook\&quot;: \&quot;${book_id}\&quot;,\n  \&quot;FirstName\&quot;: \&quot;${updated_fName}\&quot;,\n  \&quot;LastName\&quot;: \&quot;Doe\&quot;\n}&quot;,
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
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>http://fakerestapi.azurewebsites.net/api/Authors/${author_id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>e7c589e7-6df6-42bb-bb8f-4f04fdd7b675</id>
      <masked>false</masked>
      <name>author_id</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>62b3f512-9733-48c8-80e0-0811099ce83c</id>
      <masked>false</masked>
      <name>book_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ecb9e3ca-9c66-4cf3-88b6-644b699bcd43</id>
      <masked>false</masked>
      <name>updated_fName</name>
   </variables>
   <variables>
      <defaultValue>'oi'</defaultValue>
      <description></description>
      <id>29454f94-5c94-4ad4-af14-2f73a912abf5</id>
      <masked>false</masked>
      <name>updated_lName</name>
   </variables>
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
